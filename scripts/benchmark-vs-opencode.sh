#!/usr/bin/env bash
# Reproducible resource-footprint benchmark: crustly vs OpenCode.
#
# Measures cold-start time, resident memory (RSS), and installed binary
# size for both tools on the SAME machine, and writes a Markdown report.
# See differentiation-strategy-vs-opencode.md §5 for the methodology
# rationale (why these three metrics, why raw numbers get published even
# when unfavorable, why this must stay independently re-runnable).
#
# This script never fabricates a number: if a tool or binary can't be
# found, its column is marked "not measured" in the report instead of
# being guessed at or omitted silently.
#
# Usage:
#   scripts/benchmark-vs-opencode.sh [OPTIONS]
#
# Options:
#   --crustly-bin PATH    Path to a crustly binary. Default: build
#                          target/release/crustly via `cargo build --release`
#                          if it doesn't already exist.
#   --opencode-bin PATH   Path to an opencode binary. Default: `opencode`
#                          resolved from PATH. If not found, OpenCode is
#                          skipped (not faked) and the report says so.
#   --runs N              Number of timed runs per tool (default: 10).
#   --output FILE          Markdown report path. Default:
#                          benchmarks/results/<timestamp>-<hostname>.md
#   -h, --help             Show this help.
#
# Requires: bash, awk, cargo (only if building crustly). Recommended but
# optional: hyperfine (https://github.com/sharkdp/hyperfine) for precise
# cold-start timing - falls back to a manual timing loop if absent.
# RSS measurement uses `/usr/bin/time -v` (GNU time) when available,
# falling back to /proc/<pid>/status polling on Linux. Neither is
# available on macOS by default (BSD `time` has no -v) - the report says
# so explicitly rather than printing a wrong number.

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

RUNS=10
CRUSTLY_BIN=""
OPENCODE_BIN=""
OUTPUT=""

usage() {
  sed -n '2,33p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'
}

while [ $# -gt 0 ]; do
  case "$1" in
    --crustly-bin) CRUSTLY_BIN="$2"; shift 2 ;;
    --opencode-bin) OPENCODE_BIN="$2"; shift 2 ;;
    --runs) RUNS="$2"; shift 2 ;;
    --output) OUTPUT="$2"; shift 2 ;;
    -h|--help) usage; exit 0 ;;
    *) echo "[benchmark] unknown argument: $1" >&2; usage; exit 1 ;;
  esac
done

TIMESTAMP="$(date -u +%Y%m%dT%H%M%SZ)"
HOSTNAME_SAFE="$(hostname 2>/dev/null | tr -c 'A-Za-z0-9_.-' '_' || echo unknown-host)"
if [ -z "$OUTPUT" ]; then
  mkdir -p benchmarks/results
  OUTPUT="benchmarks/results/${TIMESTAMP}-${HOSTNAME_SAFE}.md"
fi

log() { echo "[benchmark] $*" >&2; }

# ── Resolve binaries ─────────────────────────────────────────────────────

if [ -z "$CRUSTLY_BIN" ]; then
  if [ -x target/release/crustly ]; then
    CRUSTLY_BIN="target/release/crustly"
  else
    log "no crustly release binary found - building (cargo build --release)"
    cargo build --release
    CRUSTLY_BIN="target/release/crustly"
  fi
fi
if [ ! -x "$CRUSTLY_BIN" ]; then
  log "crustly binary not found or not executable: $CRUSTLY_BIN"
  exit 1
fi
CRUSTLY_BIN="$(cd "$(dirname "$CRUSTLY_BIN")" && pwd)/$(basename "$CRUSTLY_BIN")"
log "crustly binary: $CRUSTLY_BIN"

OPENCODE_AVAILABLE=1
if [ -z "$OPENCODE_BIN" ]; then
  if command -v opencode >/dev/null 2>&1; then
    OPENCODE_BIN="$(command -v opencode)"
  else
    OPENCODE_AVAILABLE=0
    log "opencode not found on PATH and --opencode-bin not given - OpenCode columns will be marked 'not measured', not faked"
  fi
fi
if [ "$OPENCODE_AVAILABLE" -eq 1 ]; then
  log "opencode binary: $OPENCODE_BIN"
fi

HAVE_HYPERFINE=0
command -v hyperfine >/dev/null 2>&1 && HAVE_HYPERFINE=1

# GNU time supports -v (Maximum resident set size); BSD/macOS time does not.
HAVE_GNU_TIME=0
if command -v /usr/bin/time >/dev/null 2>&1 && /usr/bin/time -v true >/dev/null 2>&1; then
  HAVE_GNU_TIME=1
fi

OS_NAME="$(uname -s)"

# ── Cold-start timing ────────────────────────────────────────────────────
# Runs `<bin> --version` N times and reports mean/min/max wall-clock time
# in milliseconds. `--version` is used (rather than `--help`) because it
# exits immediately after minimal initialization on both tools, isolating
# process/runtime startup cost from any argument-parsing work.

manual_timing_loop() {
  local bin="$1" arg="$2" runs="$3"
  local times=()
  for _ in $(seq 1 "$runs"); do
    local start end
    start="$(date +%s%N)"
    "$bin" "$arg" >/dev/null 2>&1 || true
    end="$(date +%s%N)"
    times+=("$(( (end - start) / 1000000 ))")
  done
  printf '%s\n' "${times[@]}" | awk '
    { sum += $1; if (NR==1 || $1 < min) min = $1; if ($1 > max) max = $1; n++ }
    END { printf "mean=%.1f min=%d max=%d n=%d\n", sum/n, min, max, n }
  '
}

hyperfine_timing() {
  local bin="$1" arg="$2" runs="$3"
  local json
  json="$(mktemp)"
  hyperfine --warmup 2 --runs "$runs" --export-json "$json" -- "$bin $arg" >/dev/null 2>&1
  # hyperfine reports seconds; convert to ms for consistency with the manual path.
  awk -v RS='"mean":|"min":|"max":' 'NR>1{print}' "$json" >/dev/null 2>&1 || true
  python3 - "$json" <<'PYEOF' 2>/dev/null || {
import json, sys
d = json.load(open(sys.argv[1]))["results"][0]
print(f"mean={d['mean']*1000:.1f} min={d['min']*1000:.0f} max={d['max']*1000:.0f} n={len(d['times'])}")
PYEOF
    # Fallback if python3 is unavailable: fall back to the manual loop instead.
    manual_timing_loop "$bin" "$arg" "$runs"
  }
  rm -f "$json"
}

time_cold_start() {
  local bin="$1" arg="$2" runs="$3"
  if [ "$HAVE_HYPERFINE" -eq 1 ]; then
    hyperfine_timing "$bin" "$arg" "$runs"
  else
    manual_timing_loop "$bin" "$arg" "$runs"
  fi
}

# ── Peak RSS ──────────────────────────────────────────────────────────────

measure_rss_gnu_time() {
  local bin="$1" arg="$2"
  local out
  out="$(/usr/bin/time -v "$bin" "$arg" 2>&1 >/dev/null || true)"
  echo "$out" | awk -F': ' '/Maximum resident set size/ {print $2}'
}

measure_rss_proc_poll() {
  local bin="$1" arg="$2"
  [ "$OS_NAME" = "Linux" ] || { echo ""; return; }
  "$bin" "$arg" >/dev/null 2>&1 &
  local pid=$!
  local peak=0
  while kill -0 "$pid" 2>/dev/null; do
    if [ -r "/proc/$pid/status" ]; then
      local rss
      rss="$(awk '/VmRSS/ {print $2}' "/proc/$pid/status" 2>/dev/null || echo 0)"
      [ -n "$rss" ] && [ "$rss" -gt "$peak" ] 2>/dev/null && peak="$rss"
    fi
    sleep 0.01
  done
  wait "$pid" 2>/dev/null || true
  echo "$peak"
}

measure_rss_kib() {
  local bin="$1" arg="$2"
  if [ "$HAVE_GNU_TIME" -eq 1 ]; then
    measure_rss_gnu_time "$bin" "$arg"
  elif [ "$OS_NAME" = "Linux" ]; then
    measure_rss_proc_poll "$bin" "$arg"
  else
    echo ""
  fi
}

# ── Binary size ───────────────────────────────────────────────────────────

file_size_bytes() {
  local path="$1"
  if [ "$OS_NAME" = "Darwin" ]; then
    stat -f%z "$path"
  else
    stat -c%s "$path"
  fi
}

human_kib() {
  # Input already in KiB (as reported by GNU time -v / /proc VmRSS).
  awk -v k="$1" 'BEGIN { if (k == "" ) { print "n/a" } else { printf "%.1f MiB", k/1024 } }'
}

human_bytes() {
  # Input in bytes (as reported by stat).
  awk -v b="$1" 'BEGIN { if (b == "" ) { print "n/a" } else { printf "%.1f MiB", b/1048576 } }'
}

# ── Run measurements ─────────────────────────────────────────────────────

log "timing crustly cold start ($RUNS runs)..."
CRUSTLY_TIME="$(time_cold_start "$CRUSTLY_BIN" "--version" "$RUNS")"
log "crustly: $CRUSTLY_TIME"

log "measuring crustly peak RSS..."
CRUSTLY_RSS="$(measure_rss_kib "$CRUSTLY_BIN" "--version")"

CRUSTLY_SIZE_BYTES="$(file_size_bytes "$CRUSTLY_BIN")"

if [ "$OPENCODE_AVAILABLE" -eq 1 ]; then
  log "timing opencode cold start ($RUNS runs)..."
  OPENCODE_TIME="$(time_cold_start "$OPENCODE_BIN" "--version" "$RUNS")"
  log "opencode: $OPENCODE_TIME"

  log "measuring opencode peak RSS..."
  OPENCODE_RSS="$(measure_rss_kib "$OPENCODE_BIN" "--version")"

  OPENCODE_SIZE_BYTES="$(file_size_bytes "$OPENCODE_BIN")"
else
  OPENCODE_TIME="not measured (opencode not found)"
  OPENCODE_RSS=""
  OPENCODE_SIZE_BYTES=""
fi

# ── Report ────────────────────────────────────────────────────────────────

extract_field() { echo "$1" | grep -o "$2=[0-9.]*" | cut -d= -f2; }

{
  echo "# Benchmark: crustly vs OpenCode"
  echo
  echo "Generated: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
  echo "Host: $(hostname 2>/dev/null || echo unknown) ($OS_NAME, $(uname -m))"
  echo "Methodology: differentiation-strategy-vs-opencode.md §5"
  echo "Runs per tool: $RUNS"
  echo "Timing tool: $([ "$HAVE_HYPERFINE" -eq 1 ] && echo hyperfine || echo 'manual loop (hyperfine not installed)')"
  echo "RSS tool: $([ "$HAVE_GNU_TIME" -eq 1 ] && echo '/usr/bin/time -v' || ([ "$OS_NAME" = "Linux" ] && echo '/proc polling (approximate)' || echo 'not available on this OS'))"
  echo
  echo "## Cold start (\`--version\`, wall-clock ms)"
  echo
  echo "| Tool | Mean | Min | Max |"
  echo "|---|---|---|---|"
  echo "| crustly | $(extract_field "$CRUSTLY_TIME" mean) | $(extract_field "$CRUSTLY_TIME" min) | $(extract_field "$CRUSTLY_TIME" max) |"
  if [ "$OPENCODE_AVAILABLE" -eq 1 ]; then
    echo "| opencode | $(extract_field "$OPENCODE_TIME" mean) | $(extract_field "$OPENCODE_TIME" min) | $(extract_field "$OPENCODE_TIME" max) |"
  else
    echo "| opencode | not measured | not measured | not measured |"
  fi
  echo
  echo "## Peak resident memory (RSS)"
  echo
  echo "| Tool | Peak RSS |"
  echo "|---|---|"
  echo "| crustly | $([ -n "$CRUSTLY_RSS" ] && echo "${CRUSTLY_RSS} KiB ($(human_kib "$CRUSTLY_RSS"))" || echo "not available on this OS") |"
  if [ "$OPENCODE_AVAILABLE" -eq 1 ]; then
    echo "| opencode | $([ -n "$OPENCODE_RSS" ] && echo "${OPENCODE_RSS} KiB ($(human_kib "$OPENCODE_RSS"))" || echo "not available on this OS") |"
  else
    echo "| opencode | not measured |"
  fi
  echo
  echo "> **Note:** OpenCode's real-world footprint in normal use includes its"
  echo "> background Hono server process, not just the CLI invocation measured"
  echo "> here. A single \`--version\` call may not start that server. Anyone"
  echo "> re-running this benchmark for a public comparison should additionally"
  echo "> measure RSS during an actual interactive session for both tools, not"
  echo "> only at the \`--version\` cold-start point measured by this script."
  echo
  echo "## Binary size"
  echo
  echo "| Tool | Path | Size |"
  echo "|---|---|---|"
  echo "| crustly | \`$CRUSTLY_BIN\` | $(human_bytes "$CRUSTLY_SIZE_BYTES") ($CRUSTLY_SIZE_BYTES bytes) |"
  if [ "$OPENCODE_AVAILABLE" -eq 1 ]; then
    echo "| opencode | \`$OPENCODE_BIN\` | $(human_bytes "$OPENCODE_SIZE_BYTES") ($OPENCODE_SIZE_BYTES bytes) |"
  else
    echo "| opencode | — | not measured |"
  fi
  echo
  echo "---"
  echo
  echo "Raw numbers only - no interpretation is added by this script. See"
  echo "\`differentiation-strategy-vs-opencode.md\` §5-6 before publishing any"
  echo "of this externally: publish unfavorable results too, and re-run"
  echo "periodically since both tools evolve."
} > "$OUTPUT"

log "report written to $OUTPUT"
if [ "$OPENCODE_AVAILABLE" -eq 0 ]; then
  log "install opencode (see https://opencode.ai) and re-run with --opencode-bin, or ensure it's on PATH, to get a real comparison instead of a crustly-only report"
fi
