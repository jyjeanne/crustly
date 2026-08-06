# GGUF Model Management Improvement Plan (informed by `llamastash`)

Status: **Proposal — analysis only, no implementation started.**
Date: 2026-08-06
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

`llamastash` (MIT, "The Software shall be used for Good, not Evil" clause) is
a terminal-native launcher for local LLMs: a bearer-token-authenticated
loopback daemon that spawns `llama-server` processes on demand, backed by a
TUI and a scriptable CLI. Its full scope (multi-model concurrency, port
allocation, an OpenAI/Ollama/Anthropic-compatible proxy, five TUI themes,
hardware-aware launch presets) is out of scope for this document — see §6.
The part analyzed in depth here is its **GGUF discovery and management
subsystem**, summarized from its README/docs:

- **Discovery**: auto-scans HuggingFace cache, Ollama's blob store, and LM
  Studio's cache directory, plus user-configured extra paths; live filesystem
  watching picks up new files without a restart.
- **Metadata parsing**: reads GGUF header key-value metadata directly —
  architecture, parameter count, quantization level, native context window,
  embedded chat template, and KV-cache-aware memory estimates. Not a
  filename-convention guess.
- **Deduplication**: collapses symlinks, unifies split multi-part GGUFs
  (`-00001-of-00005.gguf` style) into one logical model, and names
  content-addressed Ollama blobs intelligibly instead of showing raw SHA
  digests.
- **Multimodal pairing**: auto-detects and pairs vision/audio projector
  (`mmproj`) files with their base model.
- **Download**: `pull <hf-repo>` with disk-space prechecks, SHA-256
  verification, and `--revision <sha>` pinning for reproducible pulls.
- **Hardware awareness**: filters/ranks discovered and recommendable models
  against detected VRAM/RAM; context auto-fit picks the largest context that
  fits instead of failing at load time.
- **Agent-facing CLI contract**: stable `--json` output, byte-stable TSV when
  piped, and a documented exit-code table (64–74 per failure class) so a
  calling agent can branch on the exit code instead of parsing text.
- **Diagnostics**: a `doctor` subcommand that always exits 0 and reports
  typed, actionable problems (missing binary, wrong `llama-server` build,
  unwritable cache dir, etc.).

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
5. **No CI-refreshed community benchmark table.** llamastash's `recommend`
   ranks against externally-maintained benchmark data; replicating that
   maintenance burden is disproportionate to the value for Crustly. A
   lighter, purely local hardware-fit heuristic (Phase M10) covers the
   useful part without the ongoing-data-pipeline cost.

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
| Hardware-aware recommend | Yes (CI benchmark data) | No | **Low** (deferred, see §3.5) |

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
`reqwest`, `sha2`, filesystem calls, all of which are already used
elsewhere in the default build) that compiles `llama_cpp_models.rs` and the
`crustly llama-cpp {list,pull,rm}` CLI subcommand independently of
`feature = "llama-cpp"`. `LlamaCppProvider` itself (the FFI-backed inference
path) stays exactly as gated as it is today. This is the one prerequisite
change every later phase benefits from: it makes "manage my GGUF files" a
zero-toolchain operation, matching llamastash's actual value proposition,
without touching the inference feature-gating decision made in
`llama-cpp-2-integration-plan.md` §3.4.

**Effort**: Small. Mostly moving `#[cfg(...)]` boundaries and updating
`Cargo.toml` feature declarations; `sha2` becomes an unconditional (or
`gguf-management`-gated) dependency instead of `llama-cpp`-gated.

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
`llama-cpp-2` itself. Read only the header (a few KB), never the tensor data,
so this stays fast and memory-light even for a 40GB model file.

Extract at minimum:
- `general.architecture`, `general.name`
- Parameter count (derivable from tensor shapes in the header, or
  `general.parameter_count` where publishers set it)
- Real quantization type (`general.file_type` / per-tensor type, not a
  filename guess — `quantization_hint_from_filename` becomes a documented
  fallback for files whose header can't be read, not the primary path)
- `*.context_length` (the model's trained/native context window)
- Whether an embedded chat template (`tokenizer.chat_template`) is present

This becomes the foundation `LocalGgufModel` builds on (replacing the
filename-only `quantization_hint`), and is what Phases M2/M4/M5/M8 consume.

**Effort**: Medium. New module (e.g. `src/llm/provider/gguf_metadata.rs`),
pure functions over `&[u8]`/`Read`, straightforward to unit test against
small hand-crafted or truncated real GGUF headers (mirroring the existing
`mock_http_server` pattern of not depending on a real multi-GB model file
in tests).

### Phase M2 — Memory/VRAM footprint estimate & context auto-fit hint

**Depends on**: M1.

Using parsed architecture/param-count/quantization plus a requested `n_ctx`,
estimate resident memory (weights + KV cache) with the standard
transformer KV-cache formula, and surface it:
- In `crustly llama-cpp list` / the TUI info panel, so a user can see
  "~4.9 GB at this context length" before loading.
- As a warning (not a hard block) when `n_gpu_layers`/`n_ctx` in
  `LlamaCppProviderConfig` looks likely to exceed detected system RAM —
  advisory only, since Crustly (correctly, per `llama-cpp-2-integration-plan.md`
  §4.11) doesn't do hardware detection today and shouldn't overreach here
  either; this is a same-order-of-magnitude estimate, not a guarantee.

**Effort**: Medium.

### Phase M3 — Multi-source discovery

**Depends on**: M0.

Extend `list_local_models` to scan, in addition to
`providers.llama_cpp.models_dir`:
- A new `providers.llama_cpp.extra_model_paths: Vec<PathBuf>` config list.
- Ollama's local blob store (`~/.ollama/models/blobs`, when the `ollama`
  feature/config is present) — read-only awareness so a model already
  pulled via Ollama shows up for llama.cpp management too, without
  duplicating storage.
- Optionally, well-known cache locations (HuggingFace's `~/.cache/huggingface/hub`,
  LM Studio's model directory) behind an opt-in config flag — auto-scanning
  directories a user didn't explicitly point Crustly at is a bigger default-
  behavior change than the rest of this plan, so this sub-item should ship
  disabled by default with a config toggle, not on by default the way
  llamastash does it.

Filesystem watching (llamastash's live-update behavior) is explicitly
**deferred, not dropped** — it's a nice-to-have once multi-source scanning
exists, lower priority than getting the scan itself right.

**Effort**: Medium.

### Phase M4 — Deduplication (symlinks, split GGUFs, Ollama blob naming)

**Depends on**: M1, M3.

- Resolve symlinks before dedup-keying so the same file reached two ways
  doesn't list twice.
- Detect the `-00001-of-000NN.gguf` split-file convention and unify each
  group into one logical `LocalGgufModel` entry (report the summed size,
  the base name, first-part path as the canonical reference).
- When a discovered file is an Ollama content-addressed blob (Phase M3),
  resolve it back to Ollama's manifest name where possible instead of
  showing a raw digest.

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
and (once M2 lands) a rough "largest model your detected RAM can hold at
default settings" line. Lower priority than M0–M7 since it's diagnostic
sugar rather than filling a functional gap, but cheap once the underlying
data (M1/M2) exists.

**Effort**: Small.

### Phase M10 — Hardware-aware `recommend` (deferred)

Explicitly **deferred**, not scheduled. llamastash's version depends on an
externally maintained, CI-refreshed benchmark dataset — replicating that
maintenance burden isn't justified by this plan's scope. If pursued later,
scope it down to a purely local heuristic (detected RAM/VRAM vs. M2's memory
estimate for already-discovered models), not a ranked catalog of models the
user doesn't have yet.

---

## 6. New dependencies

| Dependency | Used by | Why |
|---|---|---|
| None required for M0 | — | Feature-flag reorganization only |
| None required for M1 | — | Hand-rolled parser over `std::io`, consistent with keeping the management path toolchain-light (the whole point of M0) |
| None required for M2–M9 | — | Built on M1's output plus existing `reqwest`/`sha2`/`tokio::fs` already in the dependency tree |

Deliberately **not** proposing a general-purpose GGUF/tensor crate (e.g. via
`candle`) for header parsing — that would reintroduce exactly the kind of
heavy, ML-framework-shaped dependency this plan is trying to keep *out* of
the management path, for a job (reading a few KB of typed KV pairs) that
doesn't need one.

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
| M9 — `doctor` diagnostics | M0 (M1/M2 for full value) | S | Low |
| M10 — Hardware-aware recommend | M2 | — | Deferred |

Recommended starting sequence: **M0 → M1**, then M2/M3/M6/M7 in any order
(all independent once M1 lands), M4/M5/M8 last since they consume the others,
M9 whenever convenient, M10 revisited only if a concrete need for it shows up.

---

## 8. Risks

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
- **Scope creep back toward llamastash's full feature set.** §3 and §6 exist
  specifically to keep this plan bounded; any future addition should be
  checked against "does this manage GGUF files Crustly already has/wants,
  or does this reimplement a launcher/daemon/proxy Crustly doesn't need."

---

## 9. Relationship to existing docs

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
