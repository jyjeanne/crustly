# Benchmarks

Reproducible performance comparisons between Crustly and other terminal AI
coding tools. See `differentiation-strategy-vs-opencode.md` (repo root) §5-6
for why these benchmarks exist and the ground rules for publishing them
(raw numbers only, unfavorable results included, re-run periodically).

## crustly vs OpenCode

```bash
scripts/benchmark-vs-opencode.sh --help
```

Measures cold-start time, peak resident memory (RSS), and binary size for
both tools on the same machine, and writes a Markdown report to
`benchmarks/results/`.

- Builds `target/release/crustly` automatically if no crustly binary is
  given and none exists yet.
- If `opencode` isn't installed or passed via `--opencode-bin`, the report
  marks its columns "not measured" instead of guessing — install it from
  <https://opencode.ai> for a real comparison.
- Uses [`hyperfine`](https://github.com/sharkdp/hyperfine) for cold-start
  timing if available (recommended: `cargo install hyperfine` or your
  package manager), otherwise falls back to a manual timing loop.
- RSS is measured via GNU `/usr/bin/time -v` when available, or by polling
  `/proc/<pid>/status` on Linux otherwise. Not currently supported on
  macOS (BSD `time` has no `-v` equivalent) — the report says so rather
  than printing a wrong number.

`benchmarks/results/` is where reports land (`<timestamp>-<hostname>.md`).
Commit a report only once it reflects a real run on real hardware with
both tools actually installed — no placeholder/fabricated numbers.
