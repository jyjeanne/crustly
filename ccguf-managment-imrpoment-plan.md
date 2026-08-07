# GGUF Model Management Improvement Plan (informed by `llamastash`)

Status: **Proposal — analysis only, no implementation started.**
Date: 2026-08-06. Two review passes so far:
- **Review 1** (self-review against the Crustly codebase): added the
  "Development phases" execution roadmap (§8) and corrected two
  inaccuracies in the original draft (§5, M3/M4), plus the
  defensive-parsing, caching, and documentation items folded into M0/M1/M3.
- **Review 2** (re-comparison against llamastash's actual README/docs/config
  reference, not just a summary of it): corrected the Ollama-discovery
  design in M3/M4 from "scan the blob store and reverse-resolve names" to
  "read Ollama's manifests directly," added concrete LM Studio scan paths,
  and added an explicit maturity caveat (§1) plus an explicit deferred item
  (MTP speculative-decoding detection, §3) that the first pass omitted
  without saying so.
- **Update 3** (scope addition, user-requested): promoted local hardware
  detection/display and hardware-fit model selection out of the "deferred"
  bucket (M10) into concrete phases (M11/M12), grounded in llamastash's
  actual detection subsystem (`docs/architecture.md`'s per-vendor subprocess
  probe chain and VRAM-fit filter, not a re-guess) — see §1's expanded
  "Hardware awareness" bullet and the new M11/M12 sections in §5.
- **Review 3** (self-review of Update 3's new material): fixed two
  underspecified points in M11/M12 that would otherwise have been decided
  differently by whoever implemented them — when hardware detection
  actually runs (never as a side effect of a plain `list`; cached
  per-invocation, not re-probed) and what default context length M12's fit
  comparison uses (the model's own native length, capped, not an unstated
  constant). Also folded this plan's priorities into `ROADMAP.md` as the
  next milestone — see that file for the ordered, ship-ready checklist;
  this document remains the detailed design/rationale it links back to.
Scope: improve how Crustly discovers, inspects, downloads, and manages local
`.gguf` model files, informed by a feature analysis of
[`llamastash`](https://github.com/llamastash/llamastash) — a Rust TUI/CLI
purpose-built for local-GGUF lifecycle management around `llama.cpp`.

This document does **not** propose adopting llamastash's daemon/launcher/proxy
architecture. Crustly already runs inference in-process (`LlamaCppProvider`,
see `llama-cpp-2-integration-plan.md`) and already has its own TUI, config
format, and agent-facing CLI. The relevant overlap is narrower: llamastash is
excellent at the part of the problem Crustly currently does the least well —
**knowing what `.gguf` files exist, what they are, and managing them safely**
— and that part is portable without importing llamastash's process-supervision
model.

---

## 1. What was analyzed

`llamastash` (MIT © Deepu K Sasidharan, "Software for Good only, not for
military or intelligence use" clause) is a terminal-native launcher for
local LLMs: a bearer-token-authenticated loopback daemon that spawns
`llama-server` processes on demand, backed by a TUI and a scriptable CLI.
Its full scope (multi-model concurrency, port allocation, an
OpenAI/Ollama/Anthropic-compatible proxy, five TUI themes, hardware-aware
launch presets, experimental Lemonade/ds4 backends) is out of scope for this
document — see §3. The part analyzed in depth here is its **GGUF discovery
and management subsystem**, verified against its actual README, `docs/`
tree, and `config.example.yaml` reference (not a paraphrase of a paraphrase):

- **Discovery**: auto-scans three default "buckets" — HuggingFace's cache
  (`~/.cache/huggingface/hub` on Linux, `~/Library/Caches/huggingface/hub`
  on macOS), Ollama's model directory (`~/.ollama/models` on both), and LM
  Studio's cache (`~/.lmstudio/models` and `~/.cache/lm-studio/models` on
  Linux; `~/Library/Caches/LMStudio/models` and `~/.lmstudio/models` on
  macOS) — plus user-configured `model_paths`, each bucket independently
  toggleable via `disable_default_cache_paths`. Live filesystem watching
  picks up new files without a restart.
- **Metadata parsing**: reads GGUF header key-value metadata directly —
  architecture, parameter count, quantization level, native context window,
  embedded chat template, and KV-cache-aware memory estimates. Not a
  filename-convention guess. Also flags MTP (multi-token-prediction /
  speculative-decoding-capable) GGUFs with a dedicated TUI badge — see the
  explicit non-goal on this in §3, added in this review pass.
- **Deduplication**: collapses symlinks, unifies split multi-part GGUFs
  (`-00001-of-00005.gguf` style) into one logical model, and — for Ollama —
  reads the **manifest files** under `~/.ollama/models/manifests/` (which map
  a human-readable name/tag to one or more content-addressed blob hashes in
  `~/.ollama/models/blobs/`) to show the model's real name instead of a raw
  SHA digest. This is manifest-first name resolution, not blob-store
  scanning with after-the-fact guessing — a distinction this plan's first
  draft got slightly wrong (see the correction in M3/M4 below).
- **Multimodal pairing**: auto-detects and pairs vision/audio projector
  (`mmproj`) files with their base model, with distinct glyphs for vision
  (◉) vs. audio (♪) in the TUI.
- **Download**: `pull <hf-repo>` with disk-space prechecks, SHA-256
  verification, and `--revision <sha>` pinning for reproducible pulls.
- **Hardware awareness**: detection is a per-vendor subprocess probe chain
  run at startup and on a slow hotplug timer, documented in llamastash's own
  `docs/architecture.md` — `nvidia-smi --query-gpu=…` (NVIDIA, Linux/Windows),
  `rocm-smi --showmeminfo vram gtt --json` (AMD, Linux), the Win32 DXGI API
  (`IDXGIFactory1::EnumAdapters1`, AMD/Intel on Windows — a native API call,
  not a subprocess), `system_profiler SPDisplaysDataType -json` (Apple
  Metal/unified memory), and `vulkaninfo --summary` as a cross-vendor
  fallback that only recovers an adapter name, no VRAM figure; the first
  probe to return non-empty data wins, and anything that returns nothing
  falls through to a `CpuOnly` classification rather than erroring. The
  result feeds three things: (1) a **VRAM-fit hard filter** that prunes any
  candidate model too large to load before any ranking happens, (2) a TUI
  host-info pane showing GPU name/VRAM/live utilization where available, and
  (3) **context auto-fit** — reading the GGUF's own attention geometry
  (`block_count`, `head_count_kv`, `head_dim`) plus the tensor table's weight
  bytes and the live free-memory snapshot to solve for the largest context
  that actually fits, rather than failing at load time or relying on
  `llama.cpp`'s own `--fit` (which llamastash's docs note misreads unified
  memory on some AMD iGPUs). Only the VRAM-fit filter and the detection
  layer itself are relevant to a *management* plan — the composite
  benchmark-score ranking on top of the filter (§3 item 5) is not.
- **Agent-facing CLI contract**: stable `--json` output, byte-stable TSV when
  piped, and a documented exit-code table (64–74 per failure class) so a
  calling agent can branch on the exit code instead of parsing text —
  reinforced by a published Agent Skills manifest (`npx skills add
  llamastash/llamastash`) that explicitly teaches calling agents to prefer
  `--json` and branch on exit codes rather than parse text output. Directly
  relevant to Phase M7 below, since Crustly is itself an agent-facing tool
  (`AGENTS.md`).
- **Diagnostics**: a `doctor` subcommand that always exits 0 and reports
  typed, actionable problems (missing binary, wrong `llama-server` build,
  unwritable cache dir, etc.).

**Project-maturity caveat**, added in this review pass: llamastash is a
single-maintainer project (119 stars, 12 forks, 4 open issues at analysis
time) with an active but still-open roadmap of its own (its `TODO.md`
lists, among other things, "llama.cpp version pinning to prevent silent
drift" and NVML-based VRAM attribution as *unfinished* work). That doesn't
undercut the specific design patterns cited above — the GGUF-format facts
(header layout, split-file convention, manifest structure) are independent
of llamastash's own maturity — but it's a reason to treat llamastash as a
useful *reference design*, not a battle-tested implementation to copy
uncritically. This plan already reflects that by hand-rolling Crustly's own
parser (M1) rather than depending on llamastash's code directly.

---

## 2. Current state in Crustly

Crustly already has a working local-inference story, and a much thinner
management layer around it. Concretely, as of this writing:

| Capability | File | Status |
|---|---|---|
| In-process GGUF inference (load + run, streaming, tool calling, grammar constraints, GPU offload, idle-unload) | `src/llm/provider/llama_cpp.rs` | **Done** — see `llama-cpp-2-integration-plan.md`, all 10 phases + 4b landed |
| List local `.gguf` files | `llama_cpp_models::list_local_models` | Directory scan of **one** configured `models_dir`, non-recursive |
| Quantization detection | `quantization_hint_from_filename` | **Filename-convention guess only** (`Q4_K_M` substring match) — never reads the GGUF header, so an unconventionally-named file reports "unknown" even though the header has the real answer |
| Model metadata (architecture, param count, context length, chat template) | — | **None.** Crustly has no GGUF header parser at all; `LlamaCppProvider` only reads what `llama-cpp-2`/`llama.cpp` itself extracts at load time, and doesn't surface it back to management commands |
| Download | `llama_cpp_models::download_model` | Direct URL or `hf:org/repo/file.gguf` shorthand, streamed to `.part`, SHA-256 verified against HF's published LFS hash when resolvable. **No disk-space precheck, no `--revision` pinning, no resume of an interrupted `.part`** |
| Delete | `llama_cpp_models::delete_model` | Single-file delete, TUI confirms first |
| Discovery scope | — | **Single directory only.** No scanning of Ollama's blob store, no HuggingFace/LM Studio cache awareness, no filesystem watching |
| Deduplication | — | **None.** Split multi-part GGUFs would list as N separate unrelated files; no symlink handling |
| Multimodal pairing | — | **None.** `mmproj` files (if present) list as ordinary unrecognized `.gguf` entries |
| CLI contract | `crustly llama-cpp {list,pull,rm}` (`src/cli/mod.rs`) | Human-readable output; no `--json`, no documented exit codes beyond the generic error path |
| TUI | Ctrl+G dialog (`src/tui/llama_cpp_download.rs`), Ctrl+O info panel | Shows path/size/filename-guessed quantization only — same gaps as the CLI, just rendered |
| Diagnostics | — | **None.** A misconfigured `models_dir`, missing GPU feature build, or unwritable cache dir surfaces as a plain error at the point of failure, not proactively |

**One structural finding worth flagging on its own**: `llama_cpp_models` (the
whole file-management module — list/pull/rm, no FFI involved) is compiled
only under `#[cfg(feature = "llama-cpp")]`
(`src/llm/provider/mod.rs:28-29`), the same feature that pulls in
`llama-cpp-2`/`llama-cpp-sys-2` and triggers a `cmake` build of vendored
`llama.cpp`. That means a user who only wants to **inspect or manage** `.gguf`
files they downloaded for use with an external Ollama/llama.cpp setup — never
loading one in-process — is forced through the same C++ toolchain requirement
as someone doing in-process inference. llamastash's entire value proposition
is that management doesn't need that. This is the first thing worth fixing
(Phase M0 below) because every other improvement in this plan builds on top
of it, and it's currently the biggest usability gap between "I have a
`.gguf` file" and "Crustly can tell me anything about it."

---

## 3. Explicit non-goals (not porting these)

To keep this plan additive rather than a second product living inside
Crustly:

1. **No embedded daemon/process-supervisor.** Crustly's `LlamaCppProvider`
   already loads and serves a model in-process on a dedicated worker thread
   (`llama-cpp-2-integration-plan.md` §4.4). llamastash's loopback daemon +
   per-model port allocation solves a different problem (running an
   *external* `llama-server` you point other tools at) that Crustly doesn't
   have and shouldn't grow just because llamastash has it.
2. **No OpenAI/Ollama/Anthropic-compatible HTTP proxy.** Out of scope for a
   model-management improvement; would duplicate Crustly's own
   `Provider`/`factory.rs` abstraction for no benefit to Crustly's own users.
3. **No multi-backend pluggability (Lemonade, ds4, etc.).** Crustly's
   inference backend is `llama-cpp-2`; that's a separate, already-settled
   architectural decision (ADR 0005), not something this plan reopens.
4. **No second theming/keybinding system.** Any TUI work here extends
   Crustly's existing dialogs and render pipeline, not a parallel UI stack.
5. **No CI-refreshed community benchmark table, and no catalog of models the
   user doesn't have yet.** llamastash's `recommend` composite-ranks
   candidates by `benchmark score × tok/s × params × recency` after its
   VRAM-fit filter; replicating that externally-maintained benchmark
   pipeline is disproportionate to the value for Crustly. **Narrowed by
   this plan's Update 3**: the VRAM-fit filter itself needs no benchmark
   data — it's a local calculation over already-discovered models (Phase
   M2's memory estimate) against detected hardware (Phase M11) — so that
   part is no longer deferred; see M12. What stays deferred (Phase M10) is
   specifically the benchmark-ranked, not-yet-downloaded-model catalog.
6. **No MTP (speculative-decoding-capable) GGUF detection, for now.** Added
   in this review pass — the first draft omitted this llamastash feature
   without saying why. llamastash flags MTP-capable GGUFs at discovery time
   because it can *act* on that flag (auto-enabling speculative decoding in
   its launcher). Crustly's `LlamaCppProvider` has no speculative-decoding
   implementation at all today, so detecting the flag with nothing to
   consume it is metadata for its own sake. Revisit this only alongside a
   future speculative-decoding phase in `llama-cpp-2-integration-plan.md`,
   not as a standalone management-side addition — at that point it's a
   small extra field on the M1 parser output, not a new subsystem.

---

## 4. Gap analysis

| Capability | llamastash | Crustly today | Priority |
|---|---|---|---|
| Management usable without heavy build toolchain | Yes (single static binary) | No — gated behind `llama-cpp` (cmake/C++) | **High** |
| GGUF header metadata (arch, params, ctx, chat template, real quant) | Yes | No (filename guess only) | **High** |
| Multi-source discovery (Ollama/LM Studio/HF caches + extra paths) | Yes | No (one dir) | **Medium** |
| Filesystem watch for new files | Yes | No | **Low** |
| Split-GGUF unification / symlink dedup | Yes | No | **Medium** |
| mmproj pairing | Yes | No | **Medium** |
| Disk-space precheck before download | Yes | No | **Medium** |
| `--revision` pinning | Yes | No (always `resolve/main`) | **Low** |
| Resumable download | Not clearly documented | No | **Low** |
| KV-cache memory estimate / context auto-fit | Yes | No | **Medium** |
| Agent-facing `--json` + documented exit codes | Yes | No | **Medium** (Crustly is itself agent-facing — see AGENTS.md — so this is more relevant to Crustly than a generic nice-to-have) |
| `doctor`-style diagnostics | Yes | No | **Low** |
| Hardware detection & display (GPU vendor/VRAM, RAM) | Yes (subprocess probe chain) | No | **Medium** — new, see M11 |
| Local hardware-fit filtering ("will this model I already have fit?") | Yes (VRAM-fit hard filter) | No | **Medium** — new, see M12 |
| Catalog-based, benchmark-ranked recommend (models not yet downloaded) | Yes (CI benchmark data) | No | **Low** (deferred, see §3 item 5, M10) |
| MTP/speculative-decoding-capable GGUF detection | Yes (TUI badge) | No | **N/A — explicitly deferred, see §3 item 6** (no consumer without a speculative-decoding engine, which Crustly doesn't have) |

---

## 5. Proposed phases

Phases are ordered so each one is independently shippable and earlier phases
unblock later ones (metadata parsing in particular is a dependency for most
of what follows). No phase requires the ones after it.

### Phase M0 — Decouple management from the `llama-cpp` build feature

**Problem**: listing/downloading/deleting `.gguf` files does no FFI and needs
no C++ toolchain, but is currently only compiled under `feature = "llama-cpp"`.

**Change**: introduce a lightweight `gguf-management` feature (or fold this
functionality into the crate's default build — it has no heavy dependencies:
`reqwest` is already an unconditional dependency (`Cargo.toml:53`, not
`optional = true`), and `sha2` only needs to move from `llama-cpp`-gated to
`gguf-management`-gated) that compiles independently of `feature = "llama-cpp"`:

- `src/llm/provider/llama_cpp_models.rs` (currently
  `#[cfg(feature = "llama-cpp")]` at `src/llm/provider/mod.rs:28-29`).
- `cmd_llama_cpp`'s **real** body in `src/cli/mod.rs` (currently split into
  a `#[cfg(feature = "llama-cpp")]` implementation at `src/cli/mod.rs:1250`
  and a `#[cfg(not(feature = "llama-cpp"))]` "rebuild with..." stub at
  `:1356` — only the `#[cfg]` target changes, the `Commands::LlamaCpp`/
  `LlamaCppCommands` enum definitions are already unconditional and need no
  change).

`LlamaCppProvider` itself (the FFI-backed inference path, `llama_cpp.rs`)
stays exactly as gated as it is today — this phase touches management code
only. Because `gguf-management` carries none of the cmake/C++ toolchain cost
that is the *specific, documented* reason `llama-cpp` is excluded from
`all-llm` (`Cargo.toml:161-164,180`: "unlike every other provider feature,
this one compiles native C++"), that reasoning does not apply here — add
`gguf-management` to `all-llm`, and consider it a candidate for `default`
outright, subject to confirming `reqwest`+`sha2`'s combined footprint is
acceptable in a default build.

This is the one prerequisite change every later phase benefits from: it
makes "manage my GGUF files" a zero-toolchain operation, matching
llamastash's actual value proposition, without touching the inference
feature-gating decision made in `llama-cpp-2-integration-plan.md` §3.4.

**Effort**: Small. Moving `#[cfg(...)]` boundaries across the two files
above and updating `Cargo.toml` feature declarations (new `gguf-management`
feature, `sha2` re-gated, `all-llm` extended). No logic changes — should be
a behavior-preserving refactor verifiable by running the existing test
suite unmodified under the new feature flag.

### Phase M1 — Pure-Rust GGUF header metadata parser

**Problem**: no code anywhere in Crustly reads GGUF header key-value
metadata; quantization is a filename guess and everything else (architecture,
parameter count, native context length, embedded chat template presence) is
simply unavailable outside of a loaded `LlamaCppProvider`.

**Change**: add a small, dependency-light GGUF header reader — the GGUF
format's header (magic, version, tensor count, KV metadata) is documented and
simple enough to parse directly with `std::io` (read the fixed header, then
walk typed KV pairs) rather than pull in a general-purpose GGUF/ML crate.
This keeps the dependency footprint aligned with Crustly's stated
"performance, memory efficiency, reduced resource consumption" positioning —
the same reasoning `llm-file-gguf-support.md` already applied when weighing
`llama-cpp-2` itself. Read only the header and KV/tensor-info section, never
the tensor data itself, so this stays fast and memory-light even for a
40GB model file — note this section's actual size varies (typically well
under a few MB, but large embedded tokenizer vocabularies can push it
higher), so size it dynamically from the header's own declared counts
rather than assuming a fixed small constant.

**Hardening requirement, not optional**: this parser runs over files that
may be corrupted, truncated, or actively adversarial (any `.gguf` a user
points Crustly at, including ones downloaded from an arbitrary URL via
Phase M6). It must reject rather than trust declared sizes: cap KV-pair
count, string length, and array length against sane upper bounds before
allocating, bail out cleanly on truncation instead of panicking on an
out-of-bounds read, and never `unwrap()` on attacker-influenced values.
This is the same class of defense llamastash documents for its own
downloads ("archive-bomb defenses: entry/size/ratio caps") applied to
parsing instead of extraction. Malformed input must degrade to "metadata
unavailable" (same posture as today's `quantization_hint_from_filename`
returning `None`), never a crash — this is a hard acceptance criterion
for M1, not a stretch goal.

**Caching**: once Phase M3 scans multiple directories, a `list` call can
mean re-parsing headers of many large files on every invocation. Cache
parsed metadata keyed by `(path, size, modified_at)` (already available
from the existing directory scan, no new stat calls) so an unchanged file
is never re-parsed — an in-memory cache is enough for M1; a persisted
cache (e.g. alongside Crustly's existing SQLite database) is worth
revisiting only if profiling after M3 shows it's needed.

Extract at minimum:
- `general.architecture`, `general.name`
- Parameter count (derivable from tensor shapes in the header, or
  `general.parameter_count` where publishers set it)
- Real quantization type (`general.file_type` / per-tensor type, not a
  filename guess — `quantization_hint_from_filename` becomes a documented
  fallback for files whose header can't be read, not the primary path)
- `*.context_length` (the model's trained/native context window)
- Whether an embedded chat template (`tokenizer.chat_template`) is present

This becomes the foundation `LocalGgufModel` builds on (demoting the
filename-only `quantization_hint` to an explicit fallback, not removing
it — it stays the answer when a header can't be read), and is what Phases
M2/M4/M5/M8 consume. Adding fields to `LocalGgufModel` has a known blast
radius that should be scoped into this phase rather than discovered during
M8: its two TUI mirrors, `LlamaCppModelSummary` and `LlamaCppModelDetails`
(`src/tui/llama_cpp_download.rs`), currently copy a subset of its fields
field-by-field and will need the same fields added; the existing 11
`llama_cpp_models` unit tests (`quantization_hint_recognizes_common_tags`
and siblings) must keep passing unmodified since the filename fallback
path doesn't change behavior, only priority.

**Effort**: Medium. New module (e.g. `src/llm/provider/gguf_metadata.rs`),
pure functions over `&[u8]`/`Read`, straightforward to unit test against
small hand-crafted or truncated real GGUF headers (mirroring the existing
`mock_http_server` pattern of not depending on a real multi-GB model file
in tests) — truncated/malformed fixtures double as the hardening tests
required above.

### Phase M2 — Memory/VRAM footprint estimate & context auto-fit hint

**Depends on**: M1.

Using parsed architecture/param-count/quantization plus a requested `n_ctx`,
estimate resident memory (weights + KV cache) with the standard
transformer KV-cache formula, and surface it:
- In `crustly llama-cpp list` / the TUI info panel, so a user can see
  "~4.9 GB at this context length" before loading.
- As a warning (not a hard block) when `n_gpu_layers`/`n_ctx` in
  `LlamaCppProviderConfig` looks likely to exceed available memory — this is
  a same-order-of-magnitude estimate, not a guarantee.

At the time this phase was first drafted, "available memory" meant only
whatever the user's own OS-level tools reported, since
`llama-cpp-2-integration-plan.md` §4.11 correctly kept Crustly out of the
hardware-detection business for the *inference engine itself* (no
auto-guessed `n_gpu_layers`). **Update 3 note**: Phase M11 below adds actual
hardware detection, but strictly on the management/display side — it never
feeds back into `LlamaCppProviderConfig` or auto-sets `n_gpu_layers`/`n_ctx`.
That keeps faith with §4.11's original reasoning (a config knob stays a
config knob, not a guess) while still answering "what can my hardware
run" as an advisory question, which is what this update's request is
actually asking for.

**Effort**: Medium.

### Phase M3 — Multi-source discovery

**Depends on**: M0.

Extend `list_local_models` to scan, in addition to
`providers.llama_cpp.models_dir`:
- A new `providers.llama_cpp.extra_model_paths: Vec<PathBuf>` config list.
- **Ollama, via its manifests — not its blob store.** Correction from this
  review's re-read of llamastash's actual behavior: the first draft
  proposed scanning `~/.ollama/models/blobs` directly and "resolving back"
  a content-addressed blob to a name afterward. llamastash instead reads
  `~/.ollama/models/manifests/**` (JSON files mapping a human-readable
  `name:tag` to the blob hash(es) that make it up) and resolves the name
  *at discovery time*, which is both simpler and gives a correct name
  unconditionally rather than a best-effort reverse lookup. Crustly should
  do the same: walk `~/.ollama/models/manifests/`, parse each manifest,
  and resolve straight to `~/.ollama/models/blobs/sha256-<hash>` with the
  manifest's own name attached — no separate "resolve blob to name" step
  needed later in M4. **Gate this on an explicit config opt-in and the
  path's actual presence on disk, not on Crustly's own `ollama` Cargo
  feature/config** — whether an Ollama *daemon* has left models on this
  machine is unrelated to whether this particular Crustly build was
  compiled with the `ollama` feature (which only governs talking to an
  Ollama HTTP server); conflating the two would hide real local files on a
  build that happens not to include `ollama`, and is the wrong signal even
  on one that does.
- Optionally, well-known cache locations behind an opt-in config flag —
  auto-scanning directories a user didn't explicitly point Crustly at is a
  bigger default-behavior change than the rest of this plan, so this
  sub-item should ship disabled by default with a config toggle, not on by
  default the way llamastash does it. Concrete paths (matching llamastash's
  own default buckets, confirmed against its README's path table):

  | Bucket | Linux | macOS |
  |---|---|---|
  | HuggingFace | `~/.cache/huggingface/hub` | `~/Library/Caches/huggingface/hub` |
  | LM Studio | `~/.lmstudio/models`, `~/.cache/lm-studio/models` | `~/Library/Caches/LMStudio/models`, `~/.lmstudio/models` |

  (Windows paths intentionally left unresolved here — worth confirming
  against Crustly's own existing Windows path conventions, e.g.
  `%APPDATA%`-relative, rather than assuming llamastash's Windows layout
  transfers unchanged.)

Filesystem watching (llamastash's live-update behavior) is explicitly
**deferred, not dropped** — it's a nice-to-have once multi-source scanning
exists, lower priority than getting the scan itself right.

**Effort**: Medium.

### Phase M4 — Deduplication (symlinks, split GGUFs)

**Depends on**: M1, M3.

- Resolve symlinks before dedup-keying so the same file reached two ways
  doesn't list twice.
- Detect the `-00001-of-00005.gguf` split-file convention (both numbers
  zero-padded to the same width) and unify each group into one logical
  `LocalGgufModel` entry (report the summed size, the base name, first-part
  path as the canonical reference).

Note: Ollama blob→name resolution is **not** this phase's job anymore —
per M3's correction above, manifest-based name resolution happens at
discovery time, so an Ollama-sourced entry already carries its real name by
the time M4 runs. What M4 still needs to handle for Ollama specifically:
the same underlying blob can be referenced by more than one manifest (e.g.
two tags pointing at identical weights) — dedup-key on the blob hash so
that case collapses to one entry with both names shown, rather than two
listings of the same file.

**Effort**: Medium.

### Phase M5 — `mmproj` (vision/audio projector) pairing

**Depends on**: M1.

Detect `mmproj`-pattern filenames/header hints and associate them with their
base text model in listings, so the TUI/CLI show "Qwen2.5-VL-7B (+ vision
projector)" rather than two unrelated entries. This directly benefits
Crustly's own multimodal path (`llama-cpp-2-integration-plan.md` §4.9,
`mtmd` support) once a projector is present locally.

**Effort**: Small–Medium.

### Phase M6 — Download hardening

**Depends on**: M0.

- **Disk-space precheck**: before starting a download, compare
  `Content-Length` (or a `HEAD` request) against available space on the
  target filesystem; fail fast with a clear message instead of filling the
  disk mid-download.
- **`--revision <sha>` pinning**: extend the `hf:org/repo/file.gguf`
  shorthand to `hf:org/repo/file.gguf@revision`, resolving against that
  specific commit instead of always `resolve/main` — reproducible pulls,
  matching llamastash's rationale.
- **Resume support**: if a `.part` file already exists for the target
  filename, attempt a `Range`-header resume instead of restarting from
  zero; fall back to a full restart if the server doesn't honor `Range`.

**Effort**: Medium.

### Phase M7 — Agent-facing CLI contract

**Depends on**: M1 (for `list --json` to be worth adding).

- `crustly llama-cpp list --json`: stable, documented schema (path, size,
  architecture, param count, quantization, context length, chat-template
  presence, estimated memory from M2). Crustly already positions itself as
  agent-usable (`AGENTS.md`, `crustly run "..."` non-interactive mode) so
  this is a direct extension of an existing product direction, not a new one.
- Document exit codes for `list`/`pull`/`rm` (reuse Crustly's existing error
  taxonomy rather than importing llamastash's numeric ranges verbatim —
  the point is *documented and stable*, not matching llamastash's specific
  numbers).

**Effort**: Small–Medium.

### Phase M8 — TUI enhancements

**Depends on**: M1, M2, M4, M5.

- Ctrl+G dialog: add architecture/parameter-count/quantization (real, from
  header)/estimated-memory columns; show split-GGUF groups and mmproj
  pairings as single entries with an expandable detail line.
- Ctrl+O info panel: replace the filename-guessed quantization
  (`LlamaCppModelDetails`, `src/tui/llama_cpp_download.rs`) with the
  header-parsed value, and add native context length / chat-template-present
  indicators.

**Effort**: Medium (rendering/state-machine work, following the existing
`render_llama_cpp_models` pattern noted in `llama-cpp-2-integration-plan.md`
Phase 7's corrected-assumptions note).

### Phase M9 — `crustly llama-cpp doctor`

**Depends on**: M0.

A diagnostics subcommand, always exiting 0, reporting (as structured findings,
not just log lines): whether the binary was built with `llama-cpp`/a GPU
feature, whether `models_dir` exists and is writable, available disk space,
and (once M2 and M11 land) the actually-detected hardware plus a rough
"largest model your detected RAM/VRAM can hold at default settings" line —
this is the natural place to surface M11's detection output as text, not a
reason to duplicate detection logic here. Lower priority than M0–M7 since
it's diagnostic sugar rather than filling a functional gap, but cheap once
the underlying data (M1/M2, and later M11) exists.

**Effort**: Small.

### Phase M10 — Catalog-based, benchmark-ranked `recommend` (deferred)

Explicitly **deferred**, not scheduled. llamastash's version depends on an
externally maintained, CI-refreshed benchmark dataset to rank models the
user does **not** yet have (`benchmark score × tok/s × params × recency`);
replicating that maintenance burden isn't justified by this plan's scope.
**Narrowed by Update 3**: the local, no-benchmark-data half of what
"hardware-aware recommend" means is no longer part of this deferred item —
see M12 below. If this phase is ever picked back up, it's specifically about
suggesting *new* models to download, not about ranking ones already on disk.

### Phase M11 — Hardware detection & display

**Depends on**: nothing functionally (can ship independently of M1-M9), but
most useful once M2 exists to interpret it against.

**Problem**: Crustly has no notion of what GPU/VRAM/RAM the machine it's
running on actually has. A user deciding which local `.gguf` file to load,
or what `n_gpu_layers`/`n_ctx` to set, currently has to know their own
hardware and do the arithmetic by hand.

**Change**: add a best-effort hardware probe (new
`src/llm/provider/hardware_detect.rs`), grounded in llamastash's own
documented approach (§1) rather than inventing a detection strategy from
scratch — deliberately **subprocess-based, not SDK-linked**, so this stays
consistent with M0's whole point (management-side code shouldn't need a
build-time C/C++ toolchain):

- **NVIDIA** (Linux/Windows): shell out to `nvidia-smi --query-gpu=name,memory.total,memory.used --format=csv,noheader`.
- **AMD** (Linux): shell out to `rocm-smi --showmeminfo vram gtt --json`.
- **AMD/Intel** (Windows): the Win32 DXGI API (`IDXGIFactory1::EnumAdapters1`)
  — a native API call via the `windows`/`windows-sys` crate, not a
  subprocess; no VRAM live-utilization data available this way, matching
  llamastash's own documented limitation for this path.
- **Apple** (macOS): shell out to `system_profiler SPDisplaysDataType -json`
  for unified-memory total.
- **Cross-vendor fallback**: shell out to `vulkaninfo --summary` for an
  adapter name when none of the above resolve anything.
- **CPU-only**: whatever of the above ran found nothing → report `CpuOnly`,
  not an error. System RAM total (relevant on the CPU-only and Apple-unified
  paths) comes from a small, already-common cross-platform crate rather than
  reimplementing per-OS `/proc/meminfo`/`sysctl`/WMI parsing — see §6.
- **Degrade cleanly everywhere**: a missing binary, a non-zero exit, a
  timeout (cap subprocess wait — a hung driver tool must not hang `doctor`
  or `list`), or unparseable output all fall through to "unknown," the same
  posture this plan already requires of the GGUF parser (M1) and the
  filename-based quantization fallback. Detection failure is never fatal to
  `crustly llama-cpp list`/`doctor`.

**Display**: surface the result in `crustly llama-cpp doctor` (extends M9)
and a new host-info line/panel in the Ctrl+G TUI dialog (extends M8) — name,
VRAM total (where available), system RAM total. Read-only and advisory, as
established in M2's note above: this never writes back into
`LlamaCppProviderConfig`.

**When this runs — added in this review pass, previously unspecified**:
detection is real subprocess spawning, not free, so it must never be a
side effect of a plain `crustly llama-cpp list` or of simply opening the
Ctrl+G dialog. It runs only when explicitly needed: `doctor`, `list
--best-fit` (M12), and the TUI host-info panel's *first* render per dialog
session — cached for the lifetime of that CLI invocation / TUI session
(a static `OnceLock`/`OnceCell` is enough; GPU hotplug mid-session is not a
case this plan tries to handle, unlike llamastash's persistent-daemon
hotplug timer, which doesn't have an equivalent in a one-shot CLI/TUI
process). This keeps the "clean fallback, never fatal" requirement above
from also becoming a "never slows down the common path" regression.

**Effort**: Medium — mostly plumbing (subprocess spawn + parse per vendor,
each independently testable against captured sample output) plus the
degradation-path testing the "clean fallback" requirement above demands.

### Phase M12 — Local hardware-fit filtering (the part of `recommend` that doesn't need benchmark data)

**Depends on**: M1, M2, M11.

This is the promoted subset of what was originally deferred as part of
Phase M10 (see §3 item 5's Update-3 note) — the part llamastash's own VRAM-
fit hard filter does *before* its benchmark-ranking step, which needs no
external dataset because it only compares two numbers Crustly can already
compute locally: M2's per-model memory estimate and M11's detected
VRAM/RAM.

- Annotate each entry in `crustly llama-cpp list --best-fit` / the TUI
  dialog with a fit indicator against the detected hardware — e.g. **Fits**
  (comfortably under budget), **Tight** (fits but leaves little headroom),
  **Won't fit** (exceeds detected VRAM+RAM) — three states, not a
  false-precision percentage. **Default context length for this
  comparison, unspecified until this review pass**: use the model's own
  parsed native context length (M1's `*.context_length`) capped at a fixed
  ceiling (e.g. 8192) so a model advertising a 1M-token native context
  doesn't get flagged "Won't fit" against a KV-cache estimate nobody would
  actually configure by default; fall back to Crustly's existing
  `default_llama_cpp_n_ctx()` (`src/config/mod.rs`) when a model's native
  length can't be parsed. Surface the context length actually used for the
  estimate next to the fit label, not just the label alone — otherwise
  "Tight" is a number a user can't sanity-check.
- `crustly llama-cpp list --best-fit` (or equivalent flag): sort
  already-discovered local models by fit instead of name/date, so "which of
  the models I already have should I actually run" has a direct answer
  without the user doing the arithmetic themselves.
- Explicitly **not** a HuggingFace catalog search and **not** benchmark-
  ranked — both stay in deferred Phase M10. This phase only ever ranks
  models the user has already downloaded.

**Effort**: Small–Medium, once M2/M11 exist — this phase is mostly a
comparison and a sort, not new detection or parsing logic.

---

## 6. New dependencies

| Dependency | Used by | Why |
|---|---|---|
| None required for M0 | — | Feature-flag reorganization only |
| None required for M1 | — | Hand-rolled parser over `std::io`, consistent with keeping the management path toolchain-light (the whole point of M0) |
| None required for M2, M3, M4, M5, M6, M7, M8, M9, M12 | — | Built on M1's output plus existing `reqwest`/`sha2`/`tokio::fs` already in the dependency tree |
| A small cross-platform system-info crate (e.g. `sysinfo`), optional, `gguf-management`-gated | M11 (system RAM total, CPU-only fallback path) | Reimplementing per-OS `/proc/meminfo`/`sysctl`/WMI parsing by hand is exactly the kind of low-value hand-rolling this plan otherwise avoids doing (contrast with M1's parser, which *is* worth hand-rolling because no lightweight crate does GGUF header parsing specifically). A RAM-total query is commodity functionality a well-maintained pure-Rust crate already solves portably. |
| `windows`/`windows-sys` (Windows-only, already optional-by-target) | M11 (DXGI adapter enumeration on Windows, AMD/Intel GPUs) | Native Win32 API binding, not a subprocess — matches llamastash's own documented approach for this specific path; no SDK/toolchain install required beyond what any Windows dev machine already has. |
| None (GPU vendor tools) | M11 (NVIDIA/AMD/Apple/Vulkan detection) | Subprocess calls to already-installed vendor tools (`nvidia-smi`, `rocm-smi`, `system_profiler`, `vulkaninfo`) via `std::process::Command` — no new Cargo dependency, and no dependency on the tool being present (degrades to "unknown" per M11's clean-degradation requirement). |

Deliberately **not** proposing a general-purpose GGUF/tensor crate (e.g. via
`candle`) for header parsing — that would reintroduce exactly the kind of
heavy, ML-framework-shaped dependency this plan is trying to keep *out* of
the management path, for a job (reading a few KB of typed KV pairs) that
doesn't need one. Likewise, deliberately **not** proposing NVML/ROCm SDK
bindings for M11 — that would reintroduce the same build-toolchain problem
M0 exists to remove, for detection that's fully served by subprocess calls
to tools GPU owners already have installed.

---

## 7. Sequencing & effort summary

| Phase | Depends on | Effort | Priority |
|---|---|---|---|
| M0 — Decouple from `llama-cpp` feature | — | S | High |
| M1 — GGUF header metadata parser | M0 | M | High |
| M2 — Memory/VRAM estimate | M1 | M | Medium |
| M3 — Multi-source discovery | M0 | M | Medium |
| M4 — Dedup (symlinks/split/blobs) | M1, M3 | M | Medium |
| M5 — mmproj pairing | M1 | S–M | Medium |
| M6 — Download hardening | M0 | M | Medium |
| M7 — Agent-facing `--json`/exit codes | M1 | S–M | Medium |
| M8 — TUI enhancements | M1, M2, M4, M5 | M | Medium |
| M9 — `doctor` diagnostics | M0 (M1/M2/M11 for full value) | S | Low |
| M10 — Catalog-based, benchmark-ranked recommend | M2, M12 | — | Deferred |
| M11 — Hardware detection & display | — (M2 to interpret against) | M | Medium |
| M12 — Local hardware-fit filtering | M1, M2, M11 | S–M | Medium |

Recommended starting sequence: **M0 → M1**, then M2/M3/M6/M7/M11 in any order
(all independent once M1 lands; M11 has no dependency on M1 at all and can
start even earlier), M4/M5/M8/M12 last since they consume the others, M9
whenever convenient, M10 revisited only if a concrete need for a
not-yet-downloaded-model catalog shows up.

---

## 8. Development phases (execution roadmap)

§5 defines *what* changes in capability terms (M0–M12); this section groups
those into shippable milestones a team can actually schedule against, each
with concrete files, acceptance criteria, and effort in developer-days
(one dev-day ≈ focused implementation + tests + review for that scope, not
wall-clock calendar time). Each milestone is independently releasable —
none blocks shipping the ones before it.

### DP1 — Foundation: unlock management without the build toolchain
**Covers**: M0, M1. **Depends on**: nothing (first milestone).

| | |
|---|---|
| Files | `Cargo.toml` (new `gguf-management` feature, `sha2` re-gate, `all-llm` extension); `src/llm/provider/mod.rs` (`#[cfg]` on `llama_cpp_models`); `src/cli/mod.rs` (`#[cfg]` on `cmd_llama_cpp`, both bodies); new `src/llm/provider/gguf_metadata.rs` |
| Tasks | Move the two `#[cfg(feature = "llama-cpp")]` boundaries to `gguf-management`; write the GGUF header parser (magic/version validation, typed KV walk, tensor-info walk for parameter-count derivation) with the bounds-checking required in M1; wire real quantization/architecture/param-count/context-length/chat-template-presence into `LocalGgufModel`, demoting the filename guess to fallback; add the in-memory metadata cache keyed by `(path, size, modified_at)`; update `LlamaCppModelSummary`/`LlamaCppModelDetails` (`src/tui/llama_cpp_download.rs`) to carry the new fields |
| Acceptance criteria | `cargo build --features gguf-management` succeeds with **no C/C++ toolchain present** (the whole point of M0) and links no `llama-cpp-2`/`llama-cpp-sys-2`; `cargo test --features gguf-management` passes, including the 11 pre-existing `llama_cpp_models` tests unmodified plus new parser tests covering a valid minimal header, a truncated file, and at least one adversarial fixture (oversized declared string/array length) that must return an error, not panic or OOM; `cargo build`/`test` with no features and with `--features llama-cpp`/`all-llm` remain green (behavior-preserving refactor + additive fields) |
| Effort | ~1 dev-day (M0) + ~4 dev-days (M1, parser + hardening + caching + downstream struct updates) ≈ **5 dev-days** |
| Exit gate | `crustly llama-cpp list` shows real architecture/quantization/context-length for a locally-present `.gguf` file, on a build with no C++ toolchain installed |

### DP2 — Reach and safety: find more files, download them more safely
**Covers**: M3, M6. **Depends on**: DP1 (M3's dedup-ready listing and M6's
integrity messaging both assume the richer `LocalGgufModel` from M1;
M3/M6 themselves don't depend on each other and can be built in parallel).

| | |
|---|---|
| Files | `src/config/mod.rs` (`extra_model_paths` on `LlamaCppProviderConfig`, default-off Ollama-blob-store opt-in flag); `src/llm/provider/llama_cpp_models.rs` (multi-path scan, disk-space precheck, `Range`-header resume, `@revision` parsing in the `hf:` shorthand) |
| Tasks | Extend `list_local_models` to accept multiple directories; add Ollama manifest-based discovery per M3's correction (walk `~/.ollama/models/manifests/`, resolve each to its blob(s) and real name, gated on config opt-in + on-disk presence, independent of the `ollama` Cargo feature — not a raw blob-store scan); `HEAD`-request disk-space precheck before `download_model` starts streaming; extend `parse_hf_shorthand` to accept an optional `@revision` suffix; attempt `Range`-header resume when a matching `.part` file already exists, falling back to a full restart on a non-206 response |
| Acceptance criteria | New unit tests for: multi-directory scan de-duplicating nothing yet (dedup is DP3) but merging listings correctly; disk-space precheck rejecting a download when free space is (mock-)reported below the target size; `@revision` shorthand parsing (valid and malformed cases, mirroring the existing `parse_hf_shorthand_none_for_malformed_shorthand` test shape); resume producing an identical final file to a non-interrupted download in the existing `mock_http_server`-based test harness |
| Effort | ~2.5 dev-days (M3) + ~2.5 dev-days (M6) ≈ **5 dev-days** |
| Exit gate | A file interrupted mid-download resumes instead of restarting; a model pulled via `ollama pull` (with the opt-in flag set) appears in `crustly llama-cpp list` without being re-downloaded |

### DP3 — Intelligence: fewer duplicate/confusing entries, useful estimates
**Covers**: M2, M4, M5. **Depends on**: DP1 (metadata), DP2 (multi-source
listing is what makes dedup non-trivial — a single-directory scan rarely
produces the same model twice).

| | |
|---|---|
| Files | `src/llm/provider/gguf_metadata.rs` (memory estimator); `src/llm/provider/llama_cpp_models.rs` (symlink resolution, split-group unification, mmproj pairing) |
| Tasks | KV-cache-aware memory estimate function taking parsed architecture/params/quantization + requested `n_ctx`; symlink canonicalization before dedup-keying; `-NNNNN-of-NNNNN.gguf` group detection and merge; blob-hash dedup for Ollama entries so two manifest tags pointing at identical weights collapse to one listing with both names shown; mmproj filename/header-hint detection and pairing into the base model's entry |
| Acceptance criteria | Memory estimate within a documented order-of-magnitude tolerance against a couple of known real model/quantization combinations (recorded as a comment, not asserted exactly — hardware/build variance makes exact assertions brittle); a synthetic 3-part split group collapses to one listing entry with the summed size; a synthetic mmproj-pattern file pairs with its base-name match and stands alone (with a clear label, not silently dropped) when no match exists |
| Effort | ~2 dev-days (M2) + ~2.5 dev-days (M4) + ~1.5 dev-days (M5) ≈ **6 dev-days** |
| Exit gate | `crustly llama-cpp list` shows one entry per logical model (split parts merged, vision projector paired) with an estimated memory figure |

### DP4 — Interfaces: agents and humans both get the new data
**Covers**: M7, M8. **Depends on**: DP1–DP3 (surfaces their combined output;
gains progressively more to show as earlier milestones land, but only
strictly *requires* DP1).

| | |
|---|---|
| Files | `src/cli/mod.rs` (`--json` flag on `list`, documented exit codes); new integration test fixture for the JSON schema; `src/tui/llama_cpp_download.rs` and its `render_llama_cpp_models`-family functions (`src/tui/render.rs`) |
| Tasks | `crustly llama-cpp list --json` with a versioned, documented schema (include a `schema_version` field from the start — cheaper to add now than to retrofit once agents depend on the shape); document and stabilize exit codes for `list`/`pull`/`rm` failure modes; extend the Ctrl+G dialog with the new metadata columns and split/mmproj grouped rows; extend the Ctrl+O info panel with header-parsed quantization/context-length/chat-template indicators |
| Acceptance criteria | `--json` output round-trips through `serde_json` in a test with a fixed expected shape; a snapshot test per new TUI row/column (following the existing `render` snapshot-test pattern noted in `llama-cpp-2-integration-plan.md` Phase 7); exit codes documented in the CLI's own `--help` text or a docs table, not only in this plan |
| Effort | ~2.5 dev-days (M7) + ~3 dev-days (M8) ≈ **5.5 dev-days** |
| Exit gate | An external script can call `crustly llama-cpp list --json`, parse a stable schema, and branch on a documented exit code without reading Crustly's source |

### DP5 — Polish: diagnostics and documentation
**Covers**: M9, plus documentation/config-example updates that earlier
milestones intentionally deferred rather than scattering piecemeal.
**Depends on**: DP1–DP4 (a `doctor` command and docs are most useful once
there's a stable feature set to describe).

| | |
|---|---|
| Files | New `crustly llama-cpp doctor` subcommand in `src/cli/mod.rs`; `README.md` (`gguf-management`/new config keys); `config.toml.example` (`extra_model_paths`, Ollama-blob opt-in, revision-pinned `hf:` examples); `docs/guides/LLAMA_CPP_GUIDE.md` (management section, following the precedent `llama-cpp-2-integration-plan.md` Phase 10 already set for this same guide) |
| Tasks | `doctor` checks: build-feature detection (`gguf-management` vs `llama-cpp` vs neither), `models_dir` existence/writability, available disk space, and — once DP6 lands — the actually-detected hardware plus a "largest model your hardware can hold" line (before DP6, this line is simply omitted rather than guessed); update all three docs to describe the shipped feature set — capability by capability, not aspirationally |
| Acceptance criteria | `doctor` always exits 0 and produces structured (not free-text-only) findings, matching this plan's own §5 M9 description; docs describe only what has actually shipped by this point, cross-referencing this plan the way `llama-cpp-2-integration-plan.md`'s own Phase 10 cross-references `llm-file-gguf-support.md` rather than restating it |
| Effort | ~1.5 dev-days (M9) + ~1 dev-day (docs) ≈ **2.5 dev-days** |
| Exit gate | A new user with no prior context can run `crustly llama-cpp doctor` and the shipped guide section to get from zero to a listed, inspectable local model |

### DP6 — Hardware-aware local recommendations
**Covers**: M11, M12. **Depends on**: DP1 (M1, for M12's memory-estimate
input) and DP3 (M2, the estimator itself). Independent of DP2/DP4/DP5 —
can be built in parallel with any of them once DP1+DP3 land. This milestone
exists because a user reviewing llamastash directly asked for the "detect my
hardware and tell me which of my models fit" experience specifically, not
just the memory-estimate math DP3 already covers in isolation.

| | |
|---|---|
| Files | New `src/llm/provider/hardware_detect.rs` (or similar — probe chain + parsing per vendor); `Cargo.toml` (`sysinfo`, `windows`/`windows-sys` on Windows, both `gguf-management`-gated); `src/llm/provider/llama_cpp_models.rs` (fit annotation/sort on `list`); `src/cli/mod.rs` (`--best-fit` flag); `src/tui/llama_cpp_download.rs` and `src/cli/mod.rs`'s `doctor` (host-info display, extending M8/M9) |
| Tasks | One detection function per vendor path (NVIDIA/AMD/Windows-DXGI/Apple/Vulkan-fallback/CPU-only), each independently unit-testable against captured sample subprocess output (not a live GPU) so CI doesn't need real hardware; a timeout wrapper around every subprocess call; the `CpuOnly`/"unknown" fallback path; a once-per-invocation cache so detection never re-runs within one CLI call or TUI session; the fit-comparison function (M2 estimate, using the capped native-context-length default, vs. M11 detected budget → `Fits`/`Tight`/`Won't fit`); the `--best-fit` sort |
| Acceptance criteria | Detection degrades to `CpuOnly`/unknown (never panics, never hangs past the timeout) when a vendor tool is absent — tested by pointing the subprocess call at a nonexistent binary path, not by requiring the test runner to lack a GPU; parsing tests use captured real `nvidia-smi`/`rocm-smi`/`system_profiler` sample output (checked into the test fixtures, not generated live); fit-annotation tests cover all three states plus the "detection found nothing, fit is unknown" fourth state (must render as "unknown," never silently as "won't fit," which would be actively misleading); a plain `crustly llama-cpp list` (no `--best-fit`) spawns zero hardware-probe subprocesses — asserted directly, not just implied by the flag design |
| Effort | ~3 dev-days (M11 — four-plus detection paths, each simple but independently testable) + ~2 dev-days (M12 — comparison, annotation, sort, TUI/CLI wiring) ≈ **5 dev-days** |
| Exit gate | `crustly llama-cpp doctor` shows real detected GPU/VRAM/RAM on a machine that has `nvidia-smi`/`rocm-smi`/etc. installed, "CPU-only" cleanly on one that doesn't, and `crustly llama-cpp list --best-fit` sorts already-downloaded models with a correct Fits/Tight/Won't-fit label on each |

### Roadmap summary

| Milestone | Covers | Depends on | Effort | 
|---|---|---|---|
| DP1 — Foundation | M0, M1 | — | 5 dev-days |
| DP2 — Reach & safety | M3, M6 | DP1 | 5 dev-days |
| DP3 — Intelligence | M2, M4, M5 | DP1, DP2 | 6 dev-days |
| DP4 — Interfaces | M7, M8 | DP1 (fully: DP1–DP3) | 5.5 dev-days |
| DP5 — Polish | M9 + docs | DP1–DP4 | 2.5 dev-days |
| DP6 — Hardware-aware local recommendations | M11, M12 | DP1, DP3 (parallel to DP2/DP4/DP5) | 5 dev-days |
| **Total** | M0–M9, M11–M12 (M10 deferred, §5) | | **~29 dev-days** |

These are sequential dependency-order estimates, not a claim that one
person spends 29 consecutive days on this — DP2/DP3's sub-items (M3/M6, and
M2/M4/M5 respectively) are each parallelizable across contributors once DP1
lands, and DP6 as a whole is parallelizable against DP2/DP4/DP5 once DP1+DP3
land, same as §7's dependency graph already implies.

## 9. Risks

- **Malformed/adversarial GGUF headers** (M1): a hand-rolled binary parser
  running over files sourced from arbitrary URLs (M6) or arbitrary
  filesystem locations (M3) is a real attack surface — a crafted header
  with huge declared lengths could otherwise cause an out-of-memory
  allocation or an out-of-bounds panic. Mitigated by the bounds-checking
  and clean-degradation requirement stated as a hard acceptance criterion
  in M1, not left as a follow-up.
- **GGUF format drift**: the header format is stable and versioned, but new
  architectures periodically introduce new KV keys. Mitigate by parsing
  defensively (unknown keys are skipped, not errors) — same posture
  `llama-cpp-2-integration-plan.md` already takes toward unexpected upstream
  behavior.
- **Parameter-count estimation accuracy**: not all publishers set
  `general.parameter_count`; deriving it from tensor shapes is more reliable
  but adds parsing complexity. Acceptable to ship M1 with "unknown" as a
  valid, honest output rather than a guess, same posture as today's
  `quantization_hint_from_filename` returning `None`.
- **Subprocess-based hardware detection (M11) hanging or misbehaving**: a
  vendor tool that hangs (a wedged driver, a broken `nvidia-smi`) must not
  hang `crustly llama-cpp doctor`/`list` — mitigated by the timeout
  requirement stated as part of M11's own acceptance criteria (DP6), not
  left implicit. Spawning `$PATH`-resolved binaries is also a mild trust
  surface (a malicious entry earlier in `$PATH` than the real tool) — no
  different in kind from any other tool Crustly already shells out to
  today, but worth the same care: don't elevate privileges around these
  calls, and treat their output as untrusted text to parse defensively
  (same posture as M1's GGUF parser), not as trusted structured data.
- **Scope creep back toward llamastash's full feature set.** §3 and §6 exist
  specifically to keep this plan bounded; any future addition should be
  checked against "does this manage GGUF files Crustly already has/wants,
  or does this reimplement a launcher/daemon/proxy Crustly doesn't need."

---

## 10. Relationship to existing docs

- `llama-cpp-2-integration-plan.md` — the in-process inference engine this
  plan's management layer feeds model files to. Phase 6 of that plan is the
  starting point this document extends; nothing here revisits that plan's
  own (settled) architectural decisions.
- `llm-file-gguf-support.md` — the original Go/No-Go evaluation for
  `llama-cpp-2`; this plan reuses its dependency-minimalism reasoning (§6
  above) rather than re-litigating it.
- `ollama-rs-integration-plan.md` — precedent for the file/module naming
  conventions (`LocalModelInfo`/`PullProgress` ↔
  `LocalGgufModel`/`DownloadProgress`) this plan's new types should keep
  following.

No ADR is proposed at this stage — the phases above are additive extensions
of an already-decided architecture (in-process `llama-cpp-2`, file-based
local model management), not a new architectural decision on their own.
