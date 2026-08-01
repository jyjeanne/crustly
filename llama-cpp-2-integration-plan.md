# `llama-cpp-2` Integration Plan

Status: **Planning — not started, gated on §0.1.** No code has been
written yet; this document is the design and phasing reference for the
implementation, ready to execute once the Go/No-Go gate it inherits from
the prior feasibility study is cleared.
Branch: `claude/llama-cpp-2-integration-mfirvd`
Dependency: [`llama-cpp-2`](https://crates.io/crates/llama-cpp-2) v0.1.151
+ `llama-cpp-sys-2`, both published from
[`utilityai/llama-cpp-rs`](https://github.com/utilityai/llama-cpp-rs) (Rust
bindings over `llama.cpp`) — version/stats per `llm-file-gguf-support.md`
§2.3, re-verify at implementation time (Phase 0).
Prior art: `llm-file-gguf-support.md` (this repo) — a dedicated feasibility
study of this exact integration, dated 2026-07-21, whose benefit/cost
analysis and Go/No-Go framework this plan builds directly on rather than
duplicating (§0.1).

## 0. Summary for reviewers

Add a fourth way to run models locally, alongside the existing native
Ollama provider (`providers.ollama`), the OpenAI-compatible local shim
(`providers.openai.base_url`), and Qwen-local (`providers.qwen.base_url`):
an **in-process `llama.cpp` provider** that loads a GGUF file directly into
the `crustly` process and runs inference with no separate server to start.

This is architecturally the most different provider in the codebase so far:
every existing provider is an HTTP client (`reqwest`/`ollama-rs`) talking to
a process that is already running. `llama-cpp-2` is a native FFI binding —
`crustly` itself becomes the inference process. That changes what "the
provider" has to own: model weights in memory, a KV-cache context, GPU
memory, and a background thread, instead of just a `Client` struct. Section
3 covers the compatibility constraints this creates, and Section 4 covers
the threading model in detail — read those before starting Phase 1.

### 0.1 Go/No-Go status inherited from the prior feasibility study

`llm-file-gguf-support.md` did the benefit/cost work this plan doesn't
repeat: seven sourced benefits (§4 there — resource footprint, air-gapped
support, tool-calling reliability via grammar constraints, multimodal
parity, model-lifecycle control, onboarding simplicity, competitive
differentiation) weighed against real costs (build/toolchain complexity,
new application code, long-term maintenance burden, capabilities lost
relative to Ollama, several-weeks effort). It ends with an explicit
decision grid (its §6) and a stated conclusion:

> **NO-GO conditional** — no "hard" criterion is confirmed as of that
> study's writing (no documented air-gapped target, no documented IT
> constraint blocking daemons, no measured tool-calling failure-rate data).
> The resource-footprint benefit (Ollama's daemon holds ~1GB+ RAM at idle
> even with no model loaded — sourced from
> [ollama/ollama#7168](https://github.com/ollama/ollama/issues/7168), a
> maintainer-confirmed report) remains a real, secondary argument, but was
> judged insufficient alone to justify several weeks of engineering plus a
> new non-SemVer C++ dependency to maintain.

**What this means for this document**: this plan is the ready-to-execute
"if GO" companion the study's own §8 calls for — the technical spike,
phased build-out, and rollout referenced there. It does **not** override
that study's conclusion. Before Phase 1 (real code) starts, re-check the
three hard criteria against current product priorities, not the ones on
record when the study was written:

1. Is air-gapped / no-local-network-access a documented target use case?
2. Are target users on IT-restricted machines where installing/running a
   background daemon is blocked?
3. Has local-model tool-calling failure become measured as a real,
   significant friction point (support requests, GitHub issues, user
   reports)?

One confirmed "yes" is enough per the study's own grid to justify
proceeding past Phase 0. Absent that, this plan is deliberately left in a
state where Phase 0 alone (§13) can be run on its own — it produces exactly
the build-time/binary-size/API-surface data the decision grid's remaining
soft factors need, without committing to Phases 1+. Re-open this gate
whenever a hard criterion changes status; don't treat "the plan already
exists" as itself a reason to proceed.

## 1. Objective

Let a user run a local GGUF model (Llama, Mistral, Qwen, Gemma, Phi, and any
other `llama.cpp`-supported architecture) with:

- **No background server process** to install, start, or keep alive (unlike
  Ollama, which requires `ollama serve` running).
- **Direct control over GGUF files already on disk** — useful for users who
  already have models downloaded (e.g. from LM Studio, `huggingface-cli`, or
  a manual download) and don't want a second copy duplicated into Ollama's
  blob store.
- **GPU acceleration** (CUDA, Metal, Vulkan) compiled in as opt-in Cargo
  features, matching how `llama.cpp` itself is typically built.

This is **additive**: nothing about `providers.ollama`, `providers.openai`,
`providers.qwen`, `providers.anthropic`, or `providers.azure` changes. A
user who never configures `providers.llama_cpp` sees zero behavior change.

## 2. Why add this, given Ollama already covers local inference?

| Capability | Native Ollama (`providers.ollama`) | `llama-cpp-2` (this plan) |
|---|:---:|:---:|
| Requires a running background server | ✅ (`ollama serve`) | ❌ (in-process) |
| Load an arbitrary local `.gguf` file directly | ❌ (must `ollama create` a Modelfile first) | ✅ |
| Startup/installation footprint | Ollama binary + its own model store | Zero extra process; model store is just a directory of `.gguf` files |
| GPU backend control | Managed by Ollama, opaque to `crustly` | Direct: `n_gpu_layers`, CUDA/Metal/Vulkan feature flags chosen at compile time |
| Model swap cost | Cheap — Ollama's server manages load/unload | Expensive — reloading a GGUF into this process is a multi-second-to-minute operation (Section 4.5) |
| Multi-client sharing of one loaded model | ✅ (Ollama server can serve several clients) | ❌ (single process, single loaded model at a time) |
| Idle memory footprint (daemon running, no active chat) | **~1GB+** — Ollama's own maintainers confirm this for the daemon's "embedded runners" alone, before any model is loaded ([ollama/ollama#7168](https://github.com/ollama/ollama/issues/7168)); grows further with a `keep_alive`-resident model | **0** outside an active `crustly` session — no daemon exists to be idle |
| Cross-process model sharing | ✅ (Ollama's server holds one copy, serves every client) | ❌ — two `crustly` processes both configured with the same `providers.llama_cpp.model_path` each load their own independent copy into RAM/VRAM; no shared-weights mechanism in this plan (`llm-file-gguf-support.md` §5.4) |
| Build complexity | None (pure Rust HTTP client) | Significant — native C++ compilation (Section 3.4) |
| Process isolation on a crash | Ollama crashing drops the HTTP connection; `crustly` survives, shows a connection error | A native-code crash in `llama.cpp` can take the whole `crustly` process down — no separate process to insulate the TUI/DB/session state (§4.11) |

In short: Ollama is the better choice for most users today and remains the
recommended default. `llama-cpp-2` is for users who specifically want
zero-server local inference, already manage their own GGUF files, or want
tighter control over GPU offload than Ollama exposes. Given the build-cost
tradeoff in the row above, this is deliberately **not** proposed for the
`default` or even `all-llm` feature set — see Section 3.4. The idle-memory
row is the most concretely sourced benefit in `llm-file-gguf-support.md`
(§4.1 there) but was, on its own, judged insufficient to flip that study's
overall Go/No-Go verdict — see §0.1.

## 3. Constraints (non-negotiable)

1. No change to the `Provider` trait (`src/llm/provider/trait.rs`) — the new
   provider conforms to it as-is, like every other provider.
2. No regression to any existing provider (`ollama`, `openai`, `qwen`,
   `anthropic`, `azure`). `create_provider()`'s existing resolution order and
   defaults are unchanged for any config that doesn't set
   `providers.llama_cpp`.
3. `llama-cpp-2` is an **optional** dependency behind a Cargo feature
   (`llama-cpp`), following the existing `openai`/`aws-bedrock`/`ollama`
   pattern in `Cargo.toml`.
4. **`llama-cpp` is excluded from `all-llm`.** Every other feature in
   `all-llm` is a pure-Rust HTTP client with a fast, hermetic build. Adding
   native C++ compilation (cmake, a C/C++ toolchain, optionally CUDA/Metal
   SDKs) to `all-llm` would break that property for everyone who builds with
   it, including CI. `llama-cpp` must be opted into explicitly:
   `cargo build --features llama-cpp`.
5. All new fields on shared types (`LLMResponse`, `AgentResponse`,
   `DisplayMessage`, DB columns) are `Option<T>` defaulting to `None`/absent,
   exactly like the `perf_metrics`/`provider_name` fields added for Ollama
   (`ollama-rs-integration-plan.md` §5.2) — this provider reuses those same
   fields rather than inventing parallel ones.
6. `cargo build`/`cargo test` **without** `--features llama-cpp` must be
   completely unaffected: no new required system dependency (cmake, C++
   compiler) for anyone not opting in.
7. All existing tests continue to pass, with and without the new feature.
8. **License compatibility, confirmed, not assumed**: `llama-cpp-2` and
   `llama-cpp-sys-2` are `MIT OR Apache-2.0` (`llm-file-gguf-support.md`
   §2.2), compatible with Crustly's own `FSL-1.1-MIT` (`Cargo.toml:7`). No
   legal blocker. Re-verify at Phase 0 in case the upstream license changes
   before implementation.

### 3.1 Verified compatibility touch points

Constraint 2 above ("no regression to any existing provider") is only as
good as actually knowing every place in the codebase that assumes there is
exactly one active `Provider`, or assumes properties true of the HTTP-based
providers that may not hold for an in-process one. Rather than asserting
this, each touch point below was traced via the knowledge graph
(`knowledge/`, `docs/graph/`) and confirmed against source — the outcome
for `llama-cpp` is recorded next to each, not just "checked":

| Touch point | Where | Assumption it makes | Safe for `llama-cpp`? |
|---|---|---|---|
| `create_provider()` call sites | `cmd_chat`, `cmd_run` (`src/cli/mod.rs`) — confirmed via `create_provider.md`'s `called_by` list and cross-checked in source | Called once at startup; result is the single active provider for the process's lifetime | ✅ — no different from any other provider; §6 |
| `ModelRouter`/tier auto-routing | `AgentService::send_message`, `service.rs:872-896` | A tier switch can swap the served model on the *same* active provider, cheaply, mid-conversation | ⚠️ **Not safe as-is** — full analysis and the mitigation in §4.10 |
| `provider_icon()` | `src/tui/render.rs:89-98` | Provider names are a fixed, small known set; unknown names degrade to a generic icon | ✅ — additive match arm, unknown-name fallback already exists so even forgetting the arm degrades gracefully, not incorrectly |
| `AgentResponse`/`DisplayMessage`/DB perf fields | `service.rs:1740-1769`, migration `20260701000001_provider_perf_metrics.sql` | Fields are `Option<T>`/nullable, provider-agnostic | ✅ — confirmed in §4.8; zero schema change needed |
| Crabrace provider registry | `src/config/crabrace.rs`, ADR 0003 | Providers may optionally be registered for discovery | ✅ — confirmed no local-provider concept exists; Ollama isn't registered either, so `llama-cpp` following the same non-integration is consistent, not a gap (§11) |
| `FailoverProvider` chain | `src/llm/provider/factory.rs` (`FailoverProvider`) | Chained providers are interchangeable on `RateLimitExceeded`/`Timeout`/5xx `ApiError` | ✅ but **not recommended** — `llama-cpp` errors map to `ApiError{status:0,..}`-style codes that don't trip the `>= 500` failover check by default (mirrors `OllamaProvider`'s own documented non-retry behavior, `ollama.rs` module doc); chaining it as a failover target would silently never activate. Not a defect (no worse than Ollama today), just not a useful pairing — noted so it isn't proposed later as if it already worked. |

Every "not fully safe" row above (`ModelRouter`) has its mitigation spelled
out where the row points, not deferred to an open question — this table
exists to make the checking itself auditable, not to introduce new unowned
risk.

### 3.2 Known costs and risks (carried over from `llm-file-gguf-support.md` §5)

The feasibility study's cost analysis is not superseded by this plan — it's
the reason several design choices above look the way they do. Recorded here
once, referenced rather than re-derived elsewhere in this document:

- **Build/packaging** (study §5.1): cmake + a C++ compiler required to
  build the vendored `llama.cpp` submodule on every `cargo build --features
  llama-cpp`, breaking the project's current "just `cargo build`" promise
  for anyone who opts in. Windows needs Visual Studio Build Tools (C++) or
  MinGW; CUDA/Vulkan/ROCm/MKL each need their own SDK installed and
  discoverable. Materially larger cold-build time and binary size. This is
  exactly why constraint 4 (§3) keeps `llama-cpp` out of `all-llm`.
- **New application code** (study §5.2): a new provider, new config
  surface, a from-scratch tool-calling story (recovery heuristic and/or
  grammar constraints — §4.7), worker-thread memory/concurrency management
  (§4.3/4.4), and from-scratch model discovery/download (§8) — all
  capabilities Ollama currently provides "for free" via its own server and
  CLI. The study's own effort estimate (§5.5): **several weeks for a
  correct MVP**, versus days for an HTTP client to an already-running
  server. This plan's phase count (§13) is the concrete breakdown of that
  estimate, not a separate one.
- **Long-term maintenance** (study §5.3): `llama-cpp-2` does **not** follow
  strict SemVer — it tracks upstream `llama.cpp` as closely as possible.
  The pinned version needs more active watching on `cargo update` than this
  codebase's other dependencies, most of which do follow SemVer.
- **Process isolation / security surface** (study §5.3, expanded in §4.11
  below): running native C++ code against a `.gguf` file — a format this
  process did not produce and may not control the provenance of — in the
  **same process** as the TUI, the SQLite DB connection, and every tool
  execution is a real reduction in blast-radius compared to today, where a
  bad response from Ollama is "just" an HTTP error. §4.11 covers the
  mitigation this plan commits to (panic isolation) and is explicit about
  what it does **not** cover (genuine memory-unsafety in the native
  library, which `catch_unwind` cannot intercept).
- **Lost relative to Ollama** (study §5.4): no cross-process model sharing
  (§2's new table row above), idle-unload must be reimplemented from
  scratch rather than inherited from Ollama's `keep_alive` (§4.5 already
  does this), and Ollama's named model catalog (`llama3.2:3b`) is replaced
  by manual `.gguf` file management (§8) — a real UX downgrade for users
  used to `ollama pull <name>`, only partly offset by the `hf:org/repo/
  file.gguf` shorthand (§8.2).

None of these costs are new information relative to the study — they're
restated here so a reader of *this* document doesn't have to cross-reference
`llm-file-gguf-support.md` to know what tradeoffs Phase 0 onward is
actually signing up for.

## 4. Architecture

### 4.0 Codebase precedents (confirmed via `docs/graph/graph.json` and `knowledge/`)

Before designing anything new, the project's knowledge graph
(`GRAPHIFY_OUT=docs/graph`, per `AGENTS.md`) and OKF bundle (`knowledge/`)
were used to check whether this codebase already has patterns this plan
would otherwise invent from scratch. It does, in four places, plus one
piece of prior art that isn't in the graph at all (it's a design document,
not code) but is just as load-bearing — this plan now follows all five
rather than introducing parallel conventions or re-deriving already-done
analysis:

1. **A worker-thread-bridged-to-tokio pattern already exists.**
   `src/app/mod.rs:38-100`, `start_file_watcher()`, spawns a dedicated
   `std::thread::spawn` to own a synchronous, non-async library (the
   `notify::Watcher`), and bridges its output into async code via a
   `tokio::sync::mpsc::channel`, consumed by a separate `tokio::spawn` task.
   The module doc even says it explicitly: *"Spawn the synchronous watcher
   on a dedicated OS thread (not tokio)"*. This is precisely the shape §4.4
   needs for the FFI decode loop — §4.3/4.4 below are now written as "follow
   `start_file_watcher`'s pattern," not as a novel design, which also means
   its existing test coverage style/expectations are a template for this
   provider's worker-thread tests (§12.2).
2. **The model-management vocabulary is already established.**
   `src/llm/provider/ollama_models.rs` defines `LocalModelInfo`,
   `PullProgress` (with `is_success()`/`fraction()`), `ModelDetails`,
   `client_for()`, and `list_models()`/`show_model()`/`delete_model()`/
   `pull_model()`. §8 mirrors this shape/naming for
   `llama_cpp_models.rs` instead of inventing new names for the same kind of
   thing.
3. **The CLI subcommand shape is already established.**
   `src/cli/mod.rs:225-234` (`Commands::Ollama { operation: OllamaCommands }`)
   and `cmd_ollama()` (`:1103` behind `#[cfg(feature = "ollama")]`, `:1197`
   behind `#[cfg(not(feature = "ollama"))]` with a "rebuild with
   `--features X`" message) is the exact template §8 follows for
   `Commands::LlamaCpp`/`cmd_llama_cpp()`.
4. **The perf-metrics DB columns are already generic, not Ollama-specific.**
   Migration `migrations/20260701000001_provider_perf_metrics.sql` added
   `sessions.provider`, `messages.provider_name`, and
   `messages.perf_metrics_json` — its own comment says *"populated by the
   native Ollama provider"* but the columns themselves are plain nullable
   `TEXT`, keyed by provider name, not an Ollama-specific schema. This
   provider needs **zero new migrations**; see §4.8.
5. **The benefit/cost/feasibility analysis is already done.**
   `llm-file-gguf-support.md` (this repo, dated 2026-07-21) is a dedicated
   study of exactly this integration — sourced benefits (§4 there),
   itemized costs (§5), and an explicit Go/No-Go decision framework (§6).
   This plan does not re-litigate any of that; §0.1, §3.2, §4.7, §4.9, and
   §4.11 below each pull a specific, load-bearing fact from it (real crate
   feature names, the `llguidance` grammar-constrained-decoding capability,
   the process-isolation cost) rather than re-deriving or guessing at them.

One place the graph was checked and came back *negative* — worth recording
so it isn't re-investigated later: `src/config/crabrace.rs` (backing
`docs/architecture/decisions/0003-crabrace-provider-registry.md`) has no
concept of a filesystem-resident local provider; `providers.ollama` was
never registered there either. `providers.llama_cpp` doesn't need Crabrace
integration for the same reason (§11).

### 4.1 Dependencies (`Cargo.toml`)

Per `llm-file-gguf-support.md` §2.1/§2.4, the upstream repo
(`utilityai/llama-cpp-rs`) publishes two crates: `llama-cpp-2` (the
high-level Rust API this plan targets) and `llama-cpp-sys-2` (raw
`bindgen` FFI, compiles the vendored `llama.cpp` submodule via the `cmake`
crate at `cargo build` time — this is the actual source of §4.2's build
requirements). As of the study, pinned at `llama-cpp-2` v0.1.151, with the
project's own Cargo features being:

```toml
# llama-cpp-2's OWN Cargo.toml (upstream, as documented in
# llm-file-gguf-support.md §2.4) — the actual feature surface this plan's
# own features map onto, not a guess:
[features]
default = ["openmp", "android-shared-stdcxx", "common"]
cuda = ["llama-cpp-sys-2/cuda"]
metal = ["llama-cpp-sys-2/metal"]
vulkan = ["llama-cpp-sys-2/vulkan"]
rocm = ["llama-cpp-sys-2/rocm"]
opencl = ["llama-cpp-sys-2/opencl"]
mkl = ["llama-cpp-sys-2/mkl"]
openmp = ["llama-cpp-sys-2/openmp"]
mtmd = ["llama-cpp-sys-2/mtmd"]                # multimodal (vision), see §4.9
llguidance = ["dep:llguidance", "dep:toktrie"] # grammar-constrained decoding, see §4.7
sampler = []
dynamic-link = ["llama-cpp-sys-2/dynamic-link"]
system-ggml = ["llama-cpp-sys-2/system-ggml"]
```

This plan's own `Cargo.toml` additions, mapping onto the above:

```toml
[dependencies]
llama-cpp-2 = { version = "0.1.151", optional = true, default-features = false }

[features]
llama-cpp = ["dep:llama-cpp-2"]
llama-cpp-cuda = ["llama-cpp", "llama-cpp-2/cuda"]
llama-cpp-metal = ["llama-cpp", "llama-cpp-2/metal"]
llama-cpp-vulkan = ["llama-cpp", "llama-cpp-2/vulkan"]
llama-cpp-rocm = ["llama-cpp", "llama-cpp-2/rocm"]
llama-cpp-opencl = ["llama-cpp", "llama-cpp-2/opencl"]
llama-cpp-mkl = ["llama-cpp", "llama-cpp-2/mkl"]
llama-cpp-multimodal = ["llama-cpp", "llama-cpp-2/mtmd"]     # §4.9, deferred past MVP
llama-cpp-llguidance = ["llama-cpp", "llama-cpp-2/llguidance"] # §4.7, deferred past MVP
# NOTE: deliberately NOT added to `all-llm` — see §3.4.
```

`default-features = false` is deliberate: upstream's own `default` set
includes `android-shared-stdcxx` (irrelevant to every platform Crustly
targets) and `openmp`/`common`, neither of which should be silently forced
on Crustly's users. **Open item for Phase 0**: `llm-file-gguf-support.md`
does not document what the `common`/`sampler`/`dynamic-link`/`system-ggml`
features actually gate — read the upstream `Cargo.toml`/docs directly
before finalizing which of them (if any) `llama-cpp` (the base feature)
needs to enable to build and run at all, rather than assuming `openmp` is
purely optional.

Exact version, MSRV impact, and confirmation that the feature names above
still match what's published (the study itself notes `llama-cpp-2` does
**not** follow strict SemVer — §3.2) must be re-checked against
crates.io/the upstream repo at implementation time, not trusted as
permanently accurate from either this document or the study — this is
Phase 0's job.

### 4.2 Build requirements (must be documented, not hidden)

Building with `--features llama-cpp` additionally requires, on the build
machine:

- A C and C++ toolchain (`cc`, `cmake` ≥ 3.14 — `llama-cpp-sys-2` builds
  `llama.cpp` from its vendored/pinned source via `cmake`). Windows needs
  Visual Studio Build Tools (C++ workload) or MinGW specifically — no
  toolchain ships by default the way it effectively does on Linux/macOS
  (`llm-file-gguf-support.md` §5.1).
- For `llama-cpp-cuda`: the CUDA toolkit matching the GPU driver.
- For `llama-cpp-metal`: Xcode command-line tools (macOS only; this feature
  is a no-op / should be excluded from non-Apple targets).
- For `llama-cpp-vulkan`: the Vulkan SDK (`VULKAN_SDK` env var discoverable
  at build time).
- For `llama-cpp-rocm`: the ROCm toolkit (Linux/AMD GPUs only).
- For `llama-cpp-opencl`: an OpenCL SDK/runtime.
- For `llama-cpp-mkl`: Intel's oneMKL, installed and discoverable.
- The `rocm`/`opencl`/`mkl` backends beyond `cuda`/`metal`/`vulkan` are
  named here because they're real upstream features (§4.1), not because
  this plan commits to shipping all six — §13 Phase 8 and open question
  §10.8 (GPU backend launch scope) still need a product decision on which
  subset, if any beyond CPU-only, ships first.

This must be called out prominently in the README and `CLAUDE.md` build
instructions the moment this feature lands, and CI must build **both**
`cargo build` (no feature — must stay fast) and, in a separate job with the
toolchain pre-installed, `cargo build --features llama-cpp` — never silently
skip the feature build, or breakage in it goes unnoticed until a user hits
it.

### 4.3 New module: `src/llm/provider/llama_cpp.rs`

Implements `Provider` for `LlamaCppProvider`, but — unlike every other
provider in this file — it cannot be a thin, cheaply-cloneable wrapper
around an HTTP client. It owns:

- The loaded `LlamaModel` (immutable weights; `llama-cpp-2` types this as
  shareable, since a model's weights are read-only once loaded).
- A **single** dedicated OS thread ("the inference worker") that owns the
  mutable `LlamaContext` (KV cache, sampler state) and is the only thing
  that ever calls into the FFI decode loop.
- An `mpsc` channel into that worker, used to submit `InferenceJob`s.

```rust
pub struct LlamaCppProvider {
    /// Handle to the inference worker thread; cloning the provider clones
    /// this sender, not the model — multiple `Arc`/clones of the provider
    /// all funnel through the same single worker and the same loaded model.
    job_tx: tokio::sync::mpsc::UnboundedSender<InferenceJob>,
    model_path: PathBuf,
    default_model_name: String, // display name, e.g. GGUF filename stem
    n_ctx: u32,
    // sampling defaults, mirroring OllamaProvider's fields (temperature,
    // top_p, top_k, repeat_penalty, seed) — see §4.6
}

enum InferenceJob {
    Complete {
        request: LLMRequest,
        respond_to: tokio::sync::oneshot::Sender<Result<LLMResponse>>,
    },
    Stream {
        request: LLMRequest,
        events_tx: tokio::sync::mpsc::UnboundedSender<Result<StreamEvent>>,
    },
    /// Explicit unload, used by the idle-timeout mechanism (§4.5) and by
    /// `Drop` to free GPU/RAM promptly rather than waiting on the OS.
    Shutdown,
}

/// Construction, following `start_file_watcher`'s bridge shape
/// (`src/app/mod.rs:38`) with the direction reversed: there, a sync thread
/// pushes events to an async consumer via `tx.blocking_send`; here, async
/// callers push jobs to a sync worker thread, which drains them via the
/// mirror-image primitive, `rx.blocking_recv()`.
fn spawn_worker(model_path: PathBuf, n_gpu_layers: u32, n_ctx: u32)
    -> tokio::sync::mpsc::UnboundedSender<InferenceJob>
{
    let (job_tx, mut job_rx) = tokio::sync::mpsc::unbounded_channel::<InferenceJob>();

    std::thread::spawn(move || {
        // Model load + LlamaContext construction happen here, on this
        // thread, exactly once — not per job.
        let backend = /* LlamaBackend::init() */;
        let model = /* LlamaModel::load_from_file(&backend, &model_path, ...) */;
        let mut ctx = /* model.new_context(&backend, n_ctx, ...) */;

        while let Some(job) = job_rx.blocking_recv() {
            // catch_unwind wraps every job (§9): an FFI panic must not take
            // this thread down and orphan every future job silently.
            match job {
                InferenceJob::Complete { request, respond_to } => {
                    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        run_complete(&model, &mut ctx, request)
                    }));
                    let _ = respond_to.send(flatten_panic(result));
                }
                InferenceJob::Stream { request, events_tx } => {
                    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        run_stream(&model, &mut ctx, request, &events_tx)
                    }));
                    if let Err(e) = flatten_panic(result) {
                        let _ = events_tx.send(Err(e));
                    }
                }
                InferenceJob::Shutdown => break,
            }
        }
        // model/ctx/backend drop here, freeing RAM/VRAM.
    });

    job_tx
}
```

`Provider::complete()`/`stream()` become thin: build the `LLMRequest` into
an `InferenceJob`, send it over the channel, and `.await` the response (a
`oneshot` for `complete`, forwarding the `mpsc` receiver as the returned
`ProviderStream` for `stream`). All actual `llama.cpp` FFI calls happen only
on the worker thread — this is the load-bearing invariant of the whole
design (§4.4 explains why), and it is the same invariant `start_file_watcher`
already enforces for `notify::Watcher` (all filesystem-watch calls happen on
its dedicated thread, never on the tokio runtime).

### 4.4 Why a dedicated worker thread, not `spawn_blocking` per request

Two structural facts about `llama-cpp-2` rule out the simpler
`tokio::task::spawn_blocking`-per-request approach used for other
CPU-bound work in this codebase:

1. **`LlamaContext` is not `Send`/`Sync`-cheap to recreate.** It holds the
   KV cache and sampler state; recreating it per request means re-processing
   the entire prompt from scratch every time (no cache reuse across turns of
   the same conversation) and, worse, competes for the same GPU/CPU memory
   if multiple `spawn_blocking` tasks tried to hold their own context
   concurrently — `llama.cpp` is not designed for concurrent decode calls
   against overlapping memory from one process.
2. **Only one generation can usefully run at a time anyway.** Unlike an
   HTTP-based provider (where the *server*, not `crustly`, handles
   concurrent request queuing), there is no separate process here to absorb
   concurrent load. A single dedicated worker thread with an unbounded
   `mpsc` queue gives correct, simple, FIFO serialization of requests
   against the one context — which matches the real hardware constraint
   (one GPU/CPU worth of compute) rather than fighting it.

This is documented as the **known, accepted limitation**: two chat sessions
both pointed at the same `LlamaCppProvider` instance will serialize, not
parallelize. This mirrors reality on the hardware this targets (a single
local GPU/CPU) and is called out in the README rather than hidden.

### 4.5 Model lifecycle: load, idle-unload, swap

Because there is no server managing this, `crustly` itself must implement
what Ollama's `keep_alive` does for free:

- **Load**: happens once, at provider construction (or lazily on first
  request — recommended, so switching `providers.llama_cpp.model_path` in
  config and restarting doesn't pay load cost if the provider is never
  actually used). Loading logs progress (`llama.cpp` writes to stderr by
  default; capture and forward through `tracing` rather than letting it
  bypass the app's logging).
- **Idle unload**: an optional `idle_unload_secs` config (default: unset =
  never auto-unload), implemented as a timer on the worker thread that sends
  itself a `Shutdown` job if no `InferenceJob` arrives within the window.
  Next request after unload pays the load cost again (documented, same
  tradeoff as Ollama's `keep_alive = "5m"`).
- **Model swap** (TUI `Ctrl+W` provider/model switch, §7.3): unlike Ollama
  where switching the model name is nearly free, here it means tearing down
  the current worker thread/context/model and starting a new one pointed at
  a different `.gguf` file — a multi-second-to-tens-of-seconds operation
  depending on file size and disk speed. The TUI must show a blocking
  "Loading model…" state for this (reusing the same progress-overlay
  pattern as the Ollama pull dialog, §7.3), **not** the instant swap users
  see with Ollama. This UX difference must be explicit in the picker (e.g.
  "⏳ this may take a while" next to `llama.cpp` entries).

### 4.6 Request/response mapping

Mirrors `OllamaProvider::to_ollama_request`/`from_ollama_response`
(`src/llm/provider/ollama.rs`) structurally, adapted to `llama-cpp-2`'s API:

- **Prompt construction**: `llama.cpp` models carry their chat template as
  GGUF metadata. Use the model's embedded template via
  `llama-cpp-2`'s chat-template application call if the pinned version
  exposes it; otherwise fall back to a bundled generic ChatML-style
  template. `providers.llama_cpp.chat_template` (raw Jinja-like string, or a
  named preset) lets a user override it — needed for models that ship
  without a usable embedded template, or where the embedded one doesn't
  match how the GGUF was actually fine-tuned.
- **Sampling**: `temperature`, `top_p`, `top_k`, `repeat_penalty`, `seed`,
  `stop` sequences → `llama-cpp-2`'s sampler chain
  (`LlamaSampler::chain(...)`, or whatever the pinned version's builder API
  is called). Mirrors `OllamaProvider`'s provider-level-default +
  per-model-override pattern (`ModelOverrides`/`overrides_for`) — reused
  as-is conceptually, since one `LlamaCppProvider` instance is bound to
  exactly one loaded model, so there is no real "per-model" axis here beyond
  the single model it holds. (If multiple `.gguf` files need distinct
  tuning, that's multiple `providers.llama_cpp` entries — see open question
  in §10.)
- **Context window**: `n_ctx` is set at context-creation time (fixed for
  the life of the loaded context, unlike Ollama's per-request `num_ctx`).
  `context_window()` returns this fixed value. Changing it requires a model
  reload (§4.5), which must be documented.
- **Tokenization / detokenization**: via the model's vocab
  (`LlamaModel::str_to_token`/`token_to_str` or equivalent in the pinned
  API) — needed both for prompt construction and for turning generated
  token IDs back into text incrementally during streaming.
- **Stop conditions**: EOS token from the model's vocab, plus any
  user-configured `stop` strings (checked against the growing detokenized
  output, same as most llama.cpp front-ends do it) and `max_tokens`/
  `n_predict`.
- **Response**: `LLMResponse` with `content: vec![ContentBlock::Text {..}]`
  (plus `ContentBlock::ToolUse` when a tool call is recovered, §4.7),
  `usage` from prompt-token-count/generated-token-count, `stop_reason`
  (`EndTurn` on EOS/stop-string, `MaxTokens` on hitting `n_predict`,
  `ToolUse` when a call is recovered), and **`perf_metrics: Some(..)`**
  (§4.8 — this is a natural fit, arguably a *better* fit than Ollama's,
  since there's no network hop to blur the numbers).

### 4.7 Tool calling (no native function-calling API in `llama.cpp`)

`llama.cpp` has no OpenAI-style structured `tool_calls` output — this is
strictly prompt-engineering plus recovery, the same problem
`OllamaProvider` already solved for chat templates that print calls as
JSON text instead of populating a structured field
(`tool_call_from_content`, `parse_tool_call_object`,
`fenced_json_blocks` in `src/llm/provider/ollama.rs`).

Plan: **extract that recovery logic into a shared module**
(`src/llm/provider/tool_call_recovery.rs`) used by both `OllamaProvider`
(replacing its private copy — behavior-preserving refactor, covered by its
existing tests) and the new `LlamaCppProvider`. This avoids a second
divergent copy of fairly intricate, security-sensitive parsing (the
existing code is deliberately strict about not firing on prose that merely
*contains* JSON — see the module's doc comments) — one implementation, one
set of tests, two callers.

The tool list offered in `LLMRequest.tools` is rendered into the system
prompt as JSON-Schema-described functions (a small prompt template, needs
tuning per model family — Llama-3-Instruct, Mistral, Qwen all respond
differently to the exact phrasing), instructing the model to answer with a
JSON object naming the function and arguments when it wants to call one.

**Not a stretch goal anymore — a concrete, deferred phase.** The first
draft of this plan treated grammar-constrained decoding as speculative,
requiring a hand-built JSON-Schema→GBNF converter. `llm-file-gguf-support.md`
§2.4/§4.3 corrects this: `llama-cpp-2` already ships a first-class
`llguidance` Cargo feature (`llguidance = ["dep:llguidance", "dep:toktrie"]`,
§4.1) wrapping [`llguidance`](https://github.com/guidance-ai/llguidance) —
grammar-constrained generation is a real, existing, documented capability
of the pinned crate, not something this plan would need to build from
scratch.

Design for the deferred `llama-cpp-llguidance` feature (§4.1): map the
already-generic `Tool.input_schema` (JSON Schema, `src/llm/provider/types.rs`
— the same schemas `to_ollama_tool()` in `ollama.rs` already converts for
Ollama's native tool format) into a constraint that restricts decoding to
only tokens that keep the output a valid instance of that schema, so the
model **cannot** emit syntactically-malformed tool-call JSON — a
categorically different guarantee than "recovered after the fact." The
study's own honest caveat (§4.3 there) carries over unchanged: grammar
constraints guarantee **syntax**, not that generation won't be truncated by
the token budget before a valid JSON value completes — so the recovery
heuristic above stays the *always-on* path (works with zero extra build
complexity, zero extra dependencies), and `llguidance` is an **additive**
reliability upgrade behind its own feature flag, not a replacement for it.
This composes cleanly with the shared-module extraction already planned:
`tool_call_recovery.rs` (used by both `OllamaProvider` and
`LlamaCppProvider`) remains the fallback/default; `llama-cpp-llguidance`,
when compiled in, additionally constrains generation before recovery is
ever needed. Sequencing: land the recovery-heuristic path in Phase 4 (§13)
as originally planned — it needs no new upstream feature and is proven by
Ollama's existing tests — and treat `llguidance` wiring as its own
follow-on phase (§13 Phase 4b) once Phase 0 confirms the feature builds
and its Rust API shape (open question §10.1).

### 4.8 Performance metrics (reuse, don't reinvent)

`llama-cpp-2` exposes per-context performance counters (prompt eval
count/time, generation count/time, load time) — the moral equivalent of
what `OllamaProvider` already reports via the **existing, provider-agnostic
`PerfMetrics` type** (`src/llm/provider/types.rs`, added for Ollama; see
`ollama-rs-integration-plan.md` §5.2). No new type is needed:

```rust
// LlamaCppProvider::from_llama_response(), mirroring
// OllamaProvider::perf_metrics_from_final_data():
PerfMetrics {
    load_duration_ms: Some(load_ms),        // one-time, 0 on a warm/already-loaded context
    prompt_eval_duration_ms: Some(prefill_ms),
    eval_duration_ms: Some(generation_ms),
    total_duration_ms: Some(prefill_ms + generation_ms),
    model_was_loaded: Some(load_ms == 0),
}
```

Because this flows through the same `LLMResponse.perf_metrics` →
`AgentResponse` → `DisplayMessage` → TUI pipeline built for Ollama, **the
TUI header tok/s badge and provider badge (§7) need no new plumbing** —
only a new `provider_icon()` entry and, if useful, an extra footer line
distinguishing "GPU offload: 32/32 layers" from Ollama's output. This is
the main payoff of Ollama's Phase 2 having made `perf_metrics` generic
instead of Ollama-specific.

**No new database migration is needed for this.** Confirmed by reading
`migrations/20260701000001_provider_perf_metrics.sql` directly: it adds
`sessions.provider TEXT`, `messages.provider_name TEXT`, and
`messages.perf_metrics_json TEXT`. The migration's own comment says these
were "populated by the native Ollama provider," but the columns are plain
nullable `TEXT` keyed by whatever string `Provider::name()` returns — there
is nothing Ollama-specific in the schema itself. `LlamaCppProvider::name()`
returning `"llama-cpp"` and serializing the same `PerfMetrics` struct into
`perf_metrics_json` is enough; Phase 5 (§13) is pure Rust, no `sqlx`
migration file.

### 4.9 Vision support

**Correction from this plan's first draft**: multimodal is not an
immature/unstable corner of `llama-cpp-2` — `llm-file-gguf-support.md`
§2.4/§4.4 found it's a real, documented, first-class Cargo feature
(`mtmd = ["llama-cpp-sys-2/mtmd"]`, §4.1) with its own example in the
upstream repo, and the study explicitly frames a native GGUF engine
*without* it as a **regression** relative to Ollama/OpenAI-compat, which
already handle vision today. This plan still defers it past the MVP, but
for scope-control reasons (keep Phases 1-5 to a mergeable, testable slice —
§13), not because the capability is shaky.

Deferred design, `llama-cpp-multimodal` feature (§4.1): LLaVA-style
multimodal needs a companion `mmproj` GGUF file alongside the main model
(`providers.llama_cpp.mmproj_path`, new optional config field) and a
different decode path (image embedding before text tokens) from the
text-only flow §4.6 describes. `supports_vision()` returns `false`
unconditionally until this lands — a real, temporary capability gap versus
Ollama/OpenAI-compat, called out explicitly (not silently) so it isn't
mistaken for parity before it exists. Phase ordering: land after the MVP
(§13 Phases 0-5) is stable and tool-calling (§4.7) is in, given the study's
own framing that this is closing a gap rather than adding a differentiator.

### 4.10 Compatibility with `ModelRouter` / prompt-tier auto-routing

This is a real compatibility hazard the OKF call-graph surfaced (`okf-rs
graph callers` over `functions/src/llm/provider/router/ModelRouter/resolve`,
cross-checked directly against `src/llm/agent/service.rs` since — see the
staleness note in §4.10.1 below — the graph itself under-reports this
function's callers) — not something either the Ollama plan or this plan's
first draft accounted for.

`AgentService` optionally holds a `model_router: Option<ModelRouter>`
(`service.rs:288`). When set, **every** `send_message` call classifies the
prompt into a `ModelTier` (`PromptAnalyzer::classify_tier`,
`src/tui/prompt_analyzer.rs:252`) and resolves it to a model id
(`service.rs:872-896`):

```rust
let (_, model_id) = router.resolve(tier);   // service.rs:892
model_id.to_string()                        // provider_name is discarded
```

Note what's discarded: `ModelRouter::resolve()` returns `(provider_name,
model_id)` (`router.rs:42-48`), but the call site only keeps `model_id`.
**`self.provider` — the `Arc<dyn Provider>` chosen once by `create_provider()`
at startup — never changes per tier.** Only the `model` string in the
outgoing `LLMRequest` changes. This is silently correct for every provider
in the codebase today, because all of them are HTTP clients addressing a
server that can serve an arbitrary named model per request with no
process-level cost: Anthropic (any Claude model id), OpenAI-compatible
(any model the endpoint hosts), Ollama (any locally-pulled model, server
loads/unloads it), Qwen/DashScope. Swapping `request.model` per message is
exactly what these providers are built to accept.

**It is not safe for `LlamaCppProvider` as designed in §4.3-§4.5.** A
`LlamaCppProvider` instance has exactly one `LlamaModel`/`LlamaContext`
loaded on its worker thread, tied to one `model_path` at construction. If a
`model_router` tier resolves to a *different* model id than the one
actually loaded:

- **Silently wrong** (if the provider ignores `request.model` and just
  answers with whatever's loaded): the user believes tier-based routing
  picked a more/less capable model; it didn't — every tier gets the same
  weights, just mislabeled in the response's `model` field.
- **Hard failure** (if the provider validates `request.model` against its
  loaded model, matching `OllamaProvider`'s stricter cousins): every tier
  except the one matching the loaded GGUF errors out, breaking chat
  entirely the moment `model_router` is configured.

Neither is acceptable. **Decision for this plan**: `LlamaCppProvider`
ignores the incoming `request.model` for routing purposes (it can only ever
serve the one loaded GGUF) but **must not silently mislabel the response**
— `LLMResponse.model`/`AgentResponse.model` report the *actually loaded*
model's `display_name` (§5), not whatever `request.model` asked for. This
makes a tier mismatch **visible** (the TUI shows the real model that
answered) rather than silently swallowed. In addition, this plan's
documentation (§13 Phase 10, README) explicitly recommends **against** naming a
`llama-cpp` model in any `model_router` tier: per §4.5, actually honoring a
tier switch would mean a multi-second-to-minute blocking model reload on
every message where the tier differs from the last — the opposite of what
tier-based routing exists for (fast, cheap dispatch). This is a
documentation/convention guard, not a code-enforced one, consistent with
`model_router` being an opt-in, currently Anthropic-oriented feature (see
`ModelRouter::default_anthropic()`, `router.rs`) that no other provider
(Ollama included) has been validated against either — this plan does not
expand scope to "fix" tier-routing generally, only to ensure `llama-cpp`
degrades visibly rather than silently if a user configures it there anyway.

#### 4.10.1 Knowledge-graph staleness note (methodology, not a code issue)

While tracing this, `knowledge/functions/src/llm/provider/factory/create_provider.md`'s
recorded `calls` list was checked against `src/llm/provider/factory.rs` and
found to **omit `try_create_ollama`**, even though `factory.rs`'s actual
`create_provider()` body calls it (§4.0 point 3; confirmed by direct source
read, not the graph). `docs/graph/graph.json`'s own `built_at_commit`
(`acf04b75`) is several commits behind this branch's `HEAD`, which explains
the gap — likely predating the Ollama provider's addition to the factory
chain, or a `cfg`-gated function the AST extraction missed in that pass.

**Practical consequence for this plan**: every call-graph fact pulled from
`knowledge/`/`docs/graph/` in this document (§4.0, §4.10 above) was
cross-checked against a direct source read before being relied on — the
graph was used to find *candidates worth checking* (e.g., "does anything
call `ModelRouter::resolve`?"), not as a final source of truth by itself.
**Action item, folded into Phase 1's exit criteria (§13)**: run
`graphify update .`/`okf-rs generate` after the new provider lands (the
Rust-code post-commit hook covers `docs/graph/` automatically per
`AGENTS.md`; `knowledge/`'s OKF bundle needs the explicit `okf-rs generate`
call), so both reflect `try_create_ollama`, the new `try_create_llama_cpp`,
and this `ModelRouter` compatibility fix before anyone queries them for the
next feature.

### 4.11 Process isolation & security surface

Flagged by `llm-file-gguf-support.md` §5.3 and its open question §7.6, and
not addressed anywhere in this plan's first draft — worth fixing before
Phase 1, not discovered during it.

**The gap**: every existing provider is an HTTP client. A malformed or
malicious response from Ollama/OpenAI/Anthropic/Qwen/Azure can, at worst,
produce a `ProviderError` — the response is untrusted *data*, parsed by
Rust code with Rust's normal memory safety. A `.gguf` file loaded by
`LlamaCppProvider` is different in kind: it's an input to a **native C++
parser and inference engine** (`llama.cpp` itself), running with the same
memory-safety guarantees as any other C++ code — none — and running
**inside the same process** as the TUI, the SQLite connection, and every
tool execution. There is no separate process boundary the way there is with
Ollama; a crash in `llama.cpp` while parsing a bad GGUF header or during
decode takes the entire `crustly` process down, including any unsaved TUI
state.

**What this plan already mitigates, and what it explicitly does not**:

- The `catch_unwind`-wrapped worker loop (§4.3, §9) catches Rust-level
  **panics** inside the FFI call boundary — an `unwrap()` on an unexpected
  return value inside `llama-cpp-2`'s own Rust wrapper code, for instance —
  and turns them into an `Err` for that one caller without taking the
  worker thread (or the process) down.
- It does **not**, and cannot, catch genuine memory-unsafety in the vendored
  C++ `llama.cpp` itself (a buffer overrun triggered by a corrupt GGUF
  header, for instance) — that class of bug is a segfault/UB, not a
  catchable Rust panic, and no amount of Rust-side wrapping changes that.
  This is a real, honest limitation to document for users, not a solved
  problem.

**Mitigations this plan commits to, scoped to what's actually achievable**:

1. **User-supplied local files are already a trust boundary Crustly
   accepts elsewhere** (reading arbitrary files via the `read_file`/`glob`
   tools, parsing arbitrary documents via `doc_parser.rs`) — loading a
   `.gguf` the user explicitly configured via `model_path` is consistent
   with that existing posture, not a new category of risk for a
   user's *own* files. The README/config docs (§13 Phase 10) say this
   plainly: pointing `llama-cpp` at a `.gguf` file means trusting that file
   to the same degree as running any other native binary/library against
   it — not a Crustly-specific risk, but one worth stating rather than
   implying false isolation.
2. **Downloaded files are a different story** and get an integrity check:
   `download_model()` (§8) verifies a SHA-256 checksum against the value
   published for that file (Hugging Face model repos commonly publish
   per-file SHA-256 in their metadata/LFS pointers) when the source exposes
   one, and surfaces a clear warning — not a silent pass — when it doesn't.
   This doesn't prevent a malicious-but-correctly-hashed file, only
   corruption/tampering-in-transit; full provenance verification (signed
   manifests, publisher identity) is out of scope for this plan, consistent
   with the study's own framing of this as an open question (§7.6 there)
   rather than a solved one.
3. **No elevated trust for `.gguf` parsing relative to any other tool
   execution**: the panic-isolation in §4.3/§9 is the same category of
   defense-in-depth this codebase already applies elsewhere (tool execution
   timeouts, sandboxed bash allowlists — `src/llm/tools/sandbox.rs`), not a
   claim that `llama-cpp` is as safe as an HTTP provider. That asymmetry is
   real and should be stated in the README (§13 Phase 10), not glossed
   over.

This section doesn't resolve the underlying risk — it can't, short of
sandboxing the whole `llama.cpp` call surface in a separate process (which
would reintroduce the process-boundary cost this plan exists to remove, or
require unrelated work like `wasm`-sandboxing the inference engine, well
outside this plan's scope). Its job is to make sure the risk is documented
and deliberately accepted, not silently absent from the plan the way it was
in this document's first draft.

## 5. Configuration (`src/config/mod.rs`)

```rust
pub struct ProviderConfigs {
    // ... existing fields unchanged ...
    #[serde(default)]
    pub llama_cpp: Option<LlamaCppProviderConfig>,
}

/// In-process `llama.cpp` provider configuration (feature = "llama-cpp").
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlamaCppProviderConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,

    /// Path to a local .gguf model file. Required — unlike Ollama, there is
    /// no name-based model registry to resolve against.
    pub model_path: PathBuf,

    /// Display name for the model (defaults to the file stem of
    /// `model_path`, e.g. "qwen2.5-coder-7b-instruct-q4_k_m").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Context window size, fixed for the life of the loaded context.
    #[serde(default = "default_llama_cpp_n_ctx")]
    pub n_ctx: u32,

    /// Number of model layers to offload to GPU. 0 = CPU only (default,
    /// always buildable). Ignored (logged once) if the binary was not built
    /// with a GPU feature (llama-cpp-cuda/-metal/-vulkan).
    #[serde(default)]
    pub n_gpu_layers: u32,

    /// CPU thread count for decode. Defaults to the number of physical cores
    /// if unset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_threads: Option<u32>,

    /// Optional chat-template override (raw template string). When unset,
    /// the model's own embedded GGUF chat template is used if present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_template: Option<String>,

    /// Sampling defaults — same rationale as OllamaProviderConfig's
    /// equivalent fields (a local model rarely behaves well on generic
    /// defaults).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,

    /// Auto-unload the model after this many idle seconds. Unset = never.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_unload_secs: Option<u64>,

    /// Directory scanned/used by the model-management commands (§8) for
    /// listing and downloading .gguf files. Defaults to a platform cache
    /// dir (e.g. ~/.cache/crustly/models on Linux).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub models_dir: Option<PathBuf>,
}
```

Environment variables, following the `OLLAMA_HOST`/`QWEN_BASE_URL`
convention already in `Config::from_env`: `LLAMA_CPP_MODEL_PATH`,
`LLAMA_CPP_N_GPU_LAYERS`.

`config.toml.example` gets a new commented `[providers.llama_cpp]` block
next to the existing `[providers.ollama]` one, explicitly noting the extra
build step (`--features llama-cpp`) it requires — a user who pastes this
block into a default build must get a clear "not compiled in" error (§6),
not a silent no-op.

## 6. Factory (`src/llm/provider/factory.rs`)

```
1. Qwen            (if providers.qwen configured)
2. Ollama native    (if providers.ollama configured)
3. llama.cpp native (if providers.llama_cpp configured)   <-- new
4. OpenAI / compat local (LM Studio, Ollama via /v1, LocalAI)
5. Gemini
6. Azure OpenAI
7. Anthropic (fallback)
```

Placed after Ollama and before OpenAI, matching the existing rationale for
Ollama's placement (`factory.rs` module doc): local, explicitly-configured
providers are tried before the more generic OpenAI-compatible fallback, so
existing `providers.openai.base_url`-only setups keep resolving exactly as
before.

`try_create_llama_cpp()` follows the same
`#[cfg(feature = "llama-cpp")]` / `#[cfg(not(feature = "llama-cpp"))]` split
already used for Ollama (`factory.rs` lines ~240–337): with the feature
compiled out, a configured-but-inert `providers.llama_cpp` section produces
a clear `anyhow::bail!` naming the missing feature and the exact rebuild
command, not a silent fall-through to the next provider.

Model **loading** (the expensive part, §4.5) happens lazily on first use,
not inside `try_create_llama_cpp()` — provider construction only spawns the
worker thread and validates that `model_path` exists and is readable,
keeping app startup fast even with `providers.llama_cpp` configured.

## 7. TUI integration

Reuses every piece of plumbing already built for Ollama's Phase 2/3
(`ollama-rs-integration-plan.md` §5, §5.7) rather than re-designing it:

1. **Provider badge / tok/s in the header**: automatic, no new code beyond
   one match arm in `provider_icon()` (`src/tui/render.rs:89-98` — currently
   `"ollama" => "🦙"`, `"openai" => "🏠"`, `"anthropic" => "🤖"`,
   `"qwen" => "🌀"`, `"azure" => "☁️"`, falling back to `"🤖"` for anything
   unmatched, e.g. `"gemini"` today): add `"llama-cpp" => "⚙️"` (or `"🦙+"` —
   bikeshed at implementation time), keyed to the **exact** string
   `LlamaCppProvider::name()` returns. That string must also be exactly what
   `factory.rs`'s `try_create_llama_cpp` names the provider and what gets
   written to `messages.provider_name`/`sessions.provider` (§4.8) — one
   canonical literal (`"llama-cpp"`), not three call sites each guessing —
   the pipeline is already generic over any provider that sets
   `perf_metrics`/`provider_name` (§4.8).
2. **Per-message perf footer**: automatic, same reason.
3. **Model Info panel** (`Ctrl+O`, already built per the Ollama plan's
   follow-up): extend to show `llama.cpp`-specific fields (GPU layers
   offloaded, quantization type parsed from the GGUF filename/metadata,
   context size) when the active provider is `llama-cpp`.
4. **Model picker / "download" dialog — cannot literally reuse `Ctrl+D`.**
   Checked `src/tui/events.rs` directly rather than assuming: `Ctrl+D` is
   already `keys::is_model_download` (`:303`, Ollama's dialog), `Ctrl+O` is
   `is_model_info` (`:308`), `Ctrl+W` is `is_provider_switch` (`:313`), and
   `Ctrl+L`/`Ctrl+N`/`Ctrl+H`/`Ctrl+K`/`Ctrl+P`/`Ctrl+Y`/`Ctrl+V`/`Ctrl+C`
   are all taken too. Two real design options, not one assumed answer:
   - **(a) New dialog, new keybinding (recommended for v1).** Add
     `keys::is_llama_cpp_models` bound to an unused combination (e.g.
     `Ctrl+G`, free per the list above — confirm against the latest
     `events.rs` at implementation time, not this list, since it will have
     grown) and a new `AppMode::LlamaCppModelPicker` (`src/tui/events.rs`,
     same family as `AppMode::ModelDownload`). Backed by a **new** module
     `src/tui/llama_cpp_download.rs` mirroring `ollama_download.rs`'s
     structure (`filter_suggestions`-equivalent over `LocalGgufModel`
     entries, a `download` sub-flow) rather than branching inside
     `ollama_download.rs` — that module's own doc comment states it is
     Ollama-specific (`src/tui/ollama_download.rs:1-11`), and adding a
     provider-conditional branch there would blur a currently
     single-purpose, well-tested file. `App` gains
     `llama_cpp_download_task: Option<tokio::task::JoinHandle<()>>`,
     mirroring `model_download_task` (`src/tui/app.rs:153`) rather than
     reusing that same field for two unrelated in-flight operations.
   - **(b) Extend the existing Provider Switch dialog (`Ctrl+W`) instead of
     adding a new key.** `src/tui/app.rs` shows this dialog is currently
     Ollama-specific end-to-end (`switch_provider_to_ollama_model`, `:2447`,
     driven by `TuiEvent::ProviderSwitchModelsListed`) — generalizing it to
     list llama.cpp `.gguf` files alongside Ollama models is more invasive
     (touches an existing, working dialog rather than adding beside it) but
     gives the user one mental model for "switch what I'm talking to"
     instead of two separate dialogs for two local backends. Both `.gguf`
     selection *and* download-a-new-model don't fit naturally into a
     "switch" dialog, though — download would still need its own surface.
   Recommendation: **(a)** for v1 — smaller diff, no risk to the existing
   Provider Switch dialog's behavior (constraint §3.2), consistent with how
   Ollama's own model-management UI is a dedicated dialog rather than folded
   into Provider Switch. Revisit unifying into (b) only if user feedback
   says having two separate "pick a local model" dialogs is confusing.
   Two sub-flows inside the new dialog, either way:
   - **Local pick**: list `.gguf` files already present under
     `providers.llama_cpp.models_dir` via `list_local_models()` (§8), let
     the user select one to become the active `model_path` (triggers the
     model-swap loading flow, §4.5, with a blocking "Loading model…"
     progress state — explicitly not instant like Ollama's swap).
   - **Download by URL/HF repo** (§8): a text field for a direct `.gguf`
     URL or a `hf:org/repo/file.gguf`-style shorthand, downloaded via
     `download_model()` into `models_dir` with a byte-progress bar (reusing
     the same `Clear` + `Block` overlay family as the Ollama pull dialog,
     driven by `DownloadProgress::fraction()` instead of Ollama's
     layer-based `PullProgress::fraction()` — different progress unit, same
     UI shell and the same method name/shape on purpose).
5. **Status bar**: `llama.cpp`-specific error surfaces — model file not
   found, out-of-memory on load (common with GPU offload misconfigured),
   GGUF version incompatible with the compiled `llama.cpp` — mapped to
   actionable messages (§4.6/§9 error mapping), not raw FFI error strings.

## 8. Model management (no server, so this is file management)

New optional module `src/llm/provider/llama_cpp_models.rs` (feature-gated),
providing what Ollama's `/api/tags`/`/api/pull`/`/api/delete` give for free
from a server, implemented here as local filesystem + HTTP download
operations. Named and shaped to match `src/llm/provider/ollama_models.rs`
field-for-field (§4.0 point 2) rather than inventing new vocabulary for the
same kind of operation:

```rust
/// A locally-present .gguf file, the llama.cpp equivalent of
/// ollama_models::LocalModelInfo (which reports installed models via
/// Ollama's /api/tags instead of a directory scan).
#[derive(Debug, Clone)]
pub struct LocalGgufModel {
    pub path: PathBuf,
    pub size_bytes: u64,
    pub modified_at: String,
    /// Best-effort guess from the filename convention (e.g. "Q4_K_M",
    /// "Q8_0") when GGUF header metadata isn't read. `None` if neither
    /// yields a match — displayed as "unknown" in the TUI/CLI rather than
    /// guessed further.
    pub quantization_hint: Option<String>,
}

/// One progress update from an in-flight `download_model` transfer.
/// The llama.cpp equivalent of ollama_models::PullProgress, adapted to
/// plain HTTP byte progress (no layer/digest concept — a .gguf download is
/// a single file, not a multi-layer manifest pull).
#[derive(Debug, Clone)]
pub struct DownloadProgress {
    pub bytes_downloaded: u64,
    pub total_bytes: Option<u64>,
}

impl DownloadProgress {
    /// Completion fraction (0.0-1.0), if the server reported Content-Length.
    /// Same clamp-and-None-on-unknown-total shape as
    /// ollama_models::PullProgress::fraction().
    pub fn fraction(&self) -> Option<f64> {
        let total = self.total_bytes.filter(|t| *t > 0)? as f64;
        (self.bytes_downloaded as f64 / total).clamp(0.0, 1.0).into()
    }
}

/// Scan `models_dir` for `*.gguf` files. Ollama's `list_models()`
/// equivalent, but a directory listing instead of an `/api/tags` call —
/// there is no `client_for()` analog because there is no server to address.
pub fn list_local_models(models_dir: &Path) -> Result<Vec<LocalGgufModel>>;

/// Download `source` (a direct URL, or an `hf:org/repo/file.gguf`
/// shorthand resolved per §8.1) into `models_dir`, streaming progress
/// through `progress_tx`. Ollama's `pull_model()` equivalent; same
/// `UnboundedSender<Progress>` callback shape, reusing the crate's existing
/// `reqwest` client rather than adding a second HTTP dependency (unlike
/// `ollama-rs`, this needs no isolated client — see the reqwest-version
/// note already in `src/llm/provider/ollama.rs`'s module doc, which does
/// not apply here).
///
/// If `expected_sha256` is `Some` (resolved from the source's published
/// metadata where available — e.g. a Hugging Face repo's LFS pointer/file
/// info, fetched alongside the download URL in §8.2's resolution step), the
/// downloaded bytes are hashed and checked before the file is left in
/// `models_dir`; a mismatch deletes the partial file and returns an error
/// naming both hashes, rather than silently keeping a corrupted/tampered
/// file. If `expected_sha256` is `None` (source has no published hash),
/// this is **not** treated as success-with-no-check: the caller (CLI/TUI)
/// surfaces an explicit "no integrity hash available for this download"
/// warning rather than staying silent about the gap — see §4.11 point 2.
pub async fn download_model(
    source: &str,
    models_dir: &Path,
    expected_sha256: Option<&str>,
    progress_tx: tokio::sync::mpsc::UnboundedSender<DownloadProgress>,
) -> Result<PathBuf>;

/// Delete a local .gguf file. Ollama's `delete_model()` equivalent.
pub fn delete_model(path: &Path) -> Result<()>;
```

No `show_model()`/`ModelDetails` equivalent in v1 — `llama.cpp` doesn't
expose a cheap "describe this GGUF" call the way Ollama's `/api/show` does;
the closest available data (architecture, parameter count, quantization) is
already surfaced via `quantization_hint` on `LocalGgufModel` and the Model
Info panel extension in §7 point 3, without a separate metadata-fetch
function. Revisit only if GGUF header parsing turns out cheap enough to
justify a dedicated call.

### 8.1 CLI subcommand

Mirrors `src/cli/mod.rs:225-234` (`Commands::Ollama { operation:
OllamaCommands }`) and `cmd_ollama()` (`:1103`/`:1197`) exactly:

```rust
/// Manage local .gguf model files (native llama.cpp provider, requires the
/// crate's 'llama-cpp' build feature)
Commands::LlamaCpp {
    #[command(subcommand)]
    operation: LlamaCppCommands,
}

pub enum LlamaCppCommands {
    /// List .gguf files in the configured models directory
    List,
    /// Download a model: a direct URL, or `hf:org/repo/file.gguf`
    Pull { source: String },
    /// Delete a local .gguf file by name or path
    Rm { model: String },
}
```

`#[cfg(feature = "llama-cpp")] async fn cmd_llama_cpp(...)` does the real
work; `#[cfg(not(feature = "llama-cpp"))]` returns the same style of error
`cmd_ollama` does without the feature: *"This build of crustly was compiled
without the 'llama-cpp' feature. Rebuild with `--features llama-cpp` to use
`crustly llama-cpp`."* A `llama_cpp_models_dir(config) -> PathBuf` helper
mirrors `ollama_host(config)` (`src/cli/mod.rs:1090`), resolving
`providers.llama_cpp.models_dir` with the same platform-cache-dir default
used in §5.

### 8.2 `hf:org/repo/file.gguf` shorthand

Resolves to `https://huggingface.co/org/repo/resolve/main/file.gguf`. This is
string-substitution only — no HuggingFace API client dependency needed for
v1. Document that gated/private HF repos aren't supported without a token
(`HF_TOKEN` env var forwarded as an `Authorization` header is a reasonable
Phase-6 addition if requested; not assumed here).

**Checksum resolution** (feeding `download_model()`'s `expected_sha256`,
§8): Hugging Face serves a file's SHA-256 via its API metadata
(`https://huggingface.co/api/models/org/repo` → file entries include an
`lfs.sha256` for LFS-tracked files, which `.gguf` files always are given
their size) — a plain unauthenticated `GET`, no HF client dependency needed,
same reasoning as the shorthand resolution itself. When the resolved file
isn't LFS-tracked or the API doesn't return a hash, `expected_sha256` is
`None` and the "no integrity hash available" warning (§4.11 point 2) is
what the user sees — not a silent skip. A direct-URL `source` (not an
`hf:` shorthand) has no metadata endpoint to query at all, so it always
downloads with `expected_sha256 = None` unless a future CLI flag lets the
user supply a known-good hash manually (not assumed in v1).

## 9. Error handling

New `llama-cpp`-specific error mapping into the existing `ProviderError`
enum (`src/llm/provider/error.rs`), analogous to `map_ollama_error`:

- Model file missing/unreadable at startup or swap → `ProviderError::
  InvalidRequest` with the resolved path and a suggestion (check
  `model_path`/`LLAMA_CPP_MODEL_PATH`).
- GGUF load failure (corrupt file, incompatible `llama.cpp` version, wrong
  quantization support not compiled in) → `ProviderError::ApiError` with
  the underlying FFI error string preserved, plus a Crustly-authored hint
  where the message is recognizable (e.g. "unknown model architecture" →
  suggest rebuilding against a newer `llama-cpp-2`).
- Out-of-memory on load (common cause: `n_gpu_layers` set higher than VRAM
  allows) → a specific, actionable variant/message rather than a generic
  panic — this must **not** unwind past the worker thread and take the
  whole process down; the worker thread catches this, reports it back
  through the `oneshot`/`mpsc` as an `Err`, and stays alive for the next
  (corrected) request rather than requiring an app restart.
- Worker thread panic (any other unexpected FFI panic) → caught via
  `std::panic::catch_unwind` at the top of the worker loop, converted to
  `ProviderError::ApiError`, thread continues; a panic in `llama.cpp` FFI
  must never take the async runtime down with it.

## 10. Open questions to settle before implementation

1. **Exact `llama-cpp-2` version and its actual API surface** for: chat
   template application, `llguidance`-backed grammar-constrained sampling
   (§4.7 — confirmed to exist as a Cargo feature by
   `llm-file-gguf-support.md`, but this plan hasn't verified its Rust API
   shape), and performance-counter access. This plan describes the *shape*
   of the integration; Phase 0 must confirm these calls exist as described
   (or adjust) against the version actually pinned.
2. **One provider instance per model, or one provider managing several
   `model_path`s with hot-swap?** This plan assumes the former (simpler,
   matches `OllamaProviderConfig`'s single-`host` shape) — multiple local
   GGUF models would mean multiple `[[providers.llama_cpp]]`-style entries
   or repeated single-model configs selected via the existing model-switch
   UX (§7 point 4). Needs a decision before §5's config shape is finalized
   if multi-model-without-reload turns out to be a common ask.
3. ~~GBNF grammar-constrained tool calling — commit to a phase number, or
   leave indefinitely as prompt-based recovery?~~ **Resolved** in §4.7: the
   `llguidance` Cargo feature is real and documented
   (`llm-file-gguf-support.md` §2.4/§4.3), so this is committed as Phase 4b
   (§13) — additive on top of the always-on recovery heuristic, not a
   replacement for it, and gated on Phase 0 confirming the feature's actual
   Rust API shape (open question 1 above).
4. **Windows support**: `llama-cpp-sys-2`'s cmake build on Windows
   (MSVC toolchain) needs an explicit CI leg before this is claimed to work
   cross-platform — don't assume parity with Linux/macOS without verifying.
5. **Should `llama-cpp` ever join `all-llm`?** Recommendation: no, keep it
   permanently separate (§3.4) given the build-time/toolchain cost — revisit
   only if `llama-cpp-sys-2` ships prebuilt binaries for common
   platform/backend combos, removing the local-compile requirement.
6. **Prebuilt release binaries**: does the project's release pipeline build
   a `llama-cpp`-enabled binary at all (and if so, which GPU backend, if
   any)? If not, `--features llama-cpp` is effectively a "build it yourself"
   feature for v1 — acceptable, but should be stated plainly in the README
   rather than implied.
7. ~~Which keybinding/dialog design for the model picker?~~ **Resolved** in
   §7 point 4 by checking `src/tui/events.rs` directly: none of the eight
   existing `Ctrl+<letter>` bindings are free for reuse, so this is a new
   `Ctrl+G`-style binding plus a new `AppMode::LlamaCppModelPicker` and a new
   `src/tui/llama_cpp_download.rs` module (option (a)), not a branch inside
   the existing Ollama-specific `ollama_download.rs`. Re-confirm the actual
   free key against `events.rs` at implementation time — new bindings may
   have been added since this plan was written.
8. **GPU backend launch scope** (`llm-file-gguf-support.md` §7.3): CPU-only
   for the first shipped release (simplest, smallest build/CI surface), or
   CPU+GPU from day one? And if GPU, which of the six real backends named
   in §4.1 (`cuda`/`metal`/`vulkan`/`rocm`/`opencl`/`mkl`) — likely
   `cuda`+`metal` first given they cover the most common dev hardware
   (NVIDIA Linux/Windows, Apple Silicon), with `vulkan`/`rocm`/`opencl`/`mkl`
   as later additions if requested. This is a product/roadmap call, not an
   engineering one — §13 Phase 8 executes whatever scope is decided here.
9. **Are any of the three "hard" Go criteria confirmed yet?**
   (`llm-file-gguf-support.md` §6/§7.1/§7.2, restated at §0.1 above.) This
   is the actual gate on whether Phase 1 should start at all — everything
   else in this open-questions list is downstream of this one being a
   "yes" for at least one criterion. Re-check against current product
   priorities before proceeding, not the answer on record when the study
   was written.
10. **Two blocking prerequisites, both from `llm-file-gguf-support.md`
    §6**, independent of how many hard criteria (§9 above) are confirmed —
    a "no" on either blocks proceeding regardless of §9's answer:
    - **Team CMake/C++ maintenance capacity** (study §7.5): does the team
      have the expertise/bandwidth to maintain a non-SemVer C++ dependency
      long-term (version bumps that can change build requirements,
      platform-specific build failures, etc.)?
    - **Engineering-weeks budget**: is a multi-week effort (§3.2/§5.5's
      estimate) available now without displacing another priority, or
      does this wait regardless of how compelling the benefits are?
11. ~~Who verifies integrity/provenance of `.gguf` files, especially
    downloaded ones?~~ **Resolved** in §4.11/§8.2: downloaded files get a
    SHA-256 check against Hugging Face's published LFS hash when available,
    with an explicit (not silent) warning when it isn't; user-supplied local
    files are treated like any other local file this codebase already reads
    (§4.11 point 1). Full provenance/signature verification remains out of
    scope, consistent with the study's own framing of this as unresolved
    beyond checksum-level integrity.

## 11. What does NOT change

- `OllamaProvider`, `OpenAIProvider`, `AnthropicProvider`, `QwenProvider`,
  `AzureOpenAIProvider`, `GeminiProvider`: no behavior or rendering changes.
- The `Provider` trait: unchanged signature.
- `create_provider()`'s default resolution with no `providers.llama_cpp`
  configured: bit-for-bit identical to today.
- `default` and `all-llm` feature builds: unaffected — no new required
  system dependency, no new required toolchain, same build time.
- TUI rendering for sessions/messages that predate this feature: unaffected
  (same `NULL`/`None` degrade-gracefully pattern as Ollama's `perf_metrics`
  rollout).
- **No database migration** — reuses the generic columns already added by
  `20260701000001_provider_perf_metrics.sql` (§4.8); no new `migrations/*.sql`
  file ships with this plan.
- **No Crabrace registry changes** (`src/config/crabrace.rs`,
  `docs/architecture/decisions/0003-crabrace-provider-registry.md`) —
  confirmed by reading that module that it has no concept of a
  filesystem-resident local provider, and `providers.ollama` was never
  registered there either (§4.0). `providers.llama_cpp` follows the same
  precedent: config-driven, not registry-discovered.
- The existing Provider Switch dialog (`Ctrl+W`,
  `switch_provider_to_ollama_model` in `src/tui/app.rs:2447`) and the
  existing Model Download dialog (`Ctrl+D`, `src/tui/ollama_download.rs`):
  unmodified — §7 point 4 adds a parallel dialog rather than branching
  inside either.

## 12. Test plan

1. **Unit tests, offline** (must not require an actual `.gguf` file or GPU):
   config parsing/defaults (`LlamaCppProviderConfig`), factory wiring
   (feature on/off branches, mirroring `try_create_ollama`'s tests),
   prompt/chat-template construction against fixture inputs, tool-call
   recovery (via the shared module from §4.7 — reuses `OllamaProvider`'s
   existing test fixtures once extracted), error mapping.
2. **Worker-thread lifecycle tests**: shutdown-on-idle-timer fires and the
   next request reloads correctly; a simulated panic inside the worker loop
   is caught and the provider stays usable for the next request (§9) — both
   testable without a real model by injecting a fake `InferenceJob` handler
   in test builds.
3. **CI constraint**: no CI runner is assumed to have GPU hardware, and
   downloading a real multi-GB GGUF model in CI is both slow and against
   most CI providers' fair-use expectations. Real-inference tests are
   `#[ignore]`d by default (matching the existing `ollama-local-llm-test-plan.md`
   precedent for Ollama, which requires a real local Ollama instance and is
   explicitly a **manual** test plan, not a CI one) and run against a small
   (~100MB-class, e.g. a tiny/toy GGUF) test model fetched once and cached,
   only in an opt-in CI job with the `llama-cpp` toolchain preinstalled.
4. **Non-regression**: `cargo test` (no features) and
   `cargo test --features all-llm` must both continue to pass unaffected —
   neither pulls in `llama-cpp` (§3.4). A separate
   `cargo test --features llama-cpp` job covers this provider specifically.
5. **Manual test plan** (written once Phase 1–5 land, same format as
   `ollama-local-llm-test-plan.md`): load a real small model
   (e.g. a Q4 quantized 1–3B model), verify chat/streaming/tool-call
   recovery/perf-metrics display/model-swap-loading-UX/idle-unload, and
   confirm the existing Ollama/OpenAI/Anthropic paths are unaffected running
   side by side in the same build.
6. **Download integrity, offline-testable** (§4.11/§8): a mocked HTTP
   response with a known body and a matching `expected_sha256` succeeds; a
   mismatched hash deletes the partial file and errors with both hashes in
   the message (never leaves a silently-corrupt file in `models_dir`); a
   response with no resolvable hash surfaces the "no integrity hash
   available" warning rather than downloading silently — same style as the
   existing `ollama_models.rs` tests that mock HTTP responses
   (`mock_server()`, `list_models_parses_tags_response`, etc.) rather than
   hitting a real network.
7. **Phase 4b non-regression** (once built): `cargo test --features
   llama-cpp` (without `llguidance`) and `cargo test --features
   llama-cpp-llguidance` both pass, and the recovery-heuristic test suite
   from Phase 4 passes identically under both — confirming `llguidance` is
   additive, not a silent behavior change for anyone who didn't opt into it.

## 13. Phasing

Each phase below lists **Deliverables** (what ships), **Files** (new or
touched — concrete paths, following §4.0's naming precedents rather than
placeholders), and **Exit criteria** (how the phase is verified done). A
phase is not "complete" until its exit criteria pass, not just its code
written — this mirrors how `ollama-rs-integration-plan.md` §0 tracks
implemented phases against honest gaps rather than intent.

### Phase 0 — Feasibility spike (no user-facing code)

Two distinct gates live in this phase, in order — a technical spike cannot
substitute for the product decision, and shouldn't be read as one:

- **Gate A — product decision (§0.1), before anything else**: confirm at
  least one hard criterion from `llm-file-gguf-support.md` §6 is a "yes"
  against *current* priorities (air-gapped/IT-restricted target; measured
  tool-calling friction — open question §10.9), or that the two blocking
  prerequisites (C++/CMake maintenance capacity, engineering-weeks budget —
  open question §10.10) are otherwise explicitly accepted by whoever owns
  that tradeoff.
  This is not an engineering task and has no "exit criteria" beyond a
  documented decision — the rest of Phase 0 (and every later phase) is
  conditional on it, not a way to arrive at it.
- **Gate B — technical spike**, once Gate A clears:
  - **Deliverables**: exact `llama-cpp-2` version pinned; a throwaway spike
    binary (not merged) that loads one small GGUF and runs a single
    `complete()`-equivalent call outside the main crate, to validate the FFI
    surface before it's load-bearing in `LlamaCppProvider`; a written
    confirmation of which of chat-template application, `llguidance`
    grammar-constrained sampling, and performance counters (§10.1) the
    pinned version actually exposes, since §4.6/§4.7/§4.8 all assume
    specific API shapes that must be checked, not assumed; confirmation of
    what the `common`/`sampler`/`dynamic-link`/`system-ggml` upstream
    features actually gate (§4.1's open item).
  - **Files**: none in `src/` — spike lives outside the crate or on a
    throwaway branch; `Cargo.toml`/`Cargo.lock` diff kept for Phase 1 to
    reuse.
  - **Exit criteria**: spike builds clean against this project's existing
    `Cargo.lock` with no version conflicts; clean-build time and binary
    size delta measured and recorded in this document's §3.4/§4.2;
    a documented answer (yes/no/how) for each of the API-surface questions
    above. A "no" on perf counters or chat templates means §4.6/§4.8 are
    revised **before** Phase 1 starts, not discovered mid-implementation.

### Phase 1 — MVP provider (CPU only, non-streaming)

- **Deliverables**: `llama-cpp` Cargo feature; `LlamaCppProvider`
  implementing `Provider::complete()` (not yet `stream()`) via the
  worker-thread architecture in §4.3/4.4 (following `start_file_watcher`,
  `src/app/mod.rs:38`); `LlamaCppProviderConfig` (§5); factory wiring
  (§6); basic error mapping (§9 — model-not-found and load-failure paths;
  the panic-`catch_unwind` hardening may land here or slip to Phase 9,
  decide when the spike's panic behavior from Phase 0 is known).
- **Files**: `Cargo.toml` (new optional dep + feature); new
  `src/llm/provider/llama_cpp.rs`; `src/config/mod.rs`
  (`LlamaCppProviderConfig`, `ProviderConfigs.llama_cpp`);
  `src/llm/provider/factory.rs` (`try_create_llama_cpp`, mirroring
  `try_create_ollama` at `factory.rs:240-337` including its
  `#[cfg(feature = "llama-cpp")]`/`#[cfg(not(...))]` split);
  `src/llm/provider/mod.rs` (module registration, mirroring the existing
  `#[cfg(feature = "ollama")]` re-export block at `mod.rs:23-37`).
- **Exit criteria**: `cargo build` (no features) unaffected — zero diff in
  build time/output; `cargo build --features llama-cpp` succeeds on at
  least Linux; a manual test loads a small real GGUF and gets a coherent
  non-streamed completion back; `cargo test` and
  `cargo test --features llama-cpp` both green; config parsing round-trips
  through `toml`; a `providers.llama_cpp` section with the feature *not*
  compiled in produces the `anyhow::bail!` naming the missing feature
  (mirrors the existing `test_disabled_*` style tests in `factory.rs`);
  `LLMResponse.model`/`AgentResponse.model` on a completion always report
  the actually-loaded GGUF's `display_name`, never a caller-requested
  `request.model` that doesn't match it (§4.10 — this is the visible-not-
  silent guard, verified here even though `model_router` integration itself
  isn't exercised until a user configures one); `okf-rs generate` and
  `graphify update .` run once against the finished phase so `knowledge/`
  and `docs/graph/` stop being stale for the next person who queries them
  (§4.10.1).

### Phase 2 — Streaming

- **Deliverables**: token-by-token generation via the worker thread into a
  `ProviderStream`, `StreamEvent` sequencing (`MessageStart` →
  `ContentBlockDelta` → `ContentBlockStop` → `MessageStop`) matching every
  other provider's `stream()`.
- **Files**: `src/llm/provider/llama_cpp.rs` only (`InferenceJob::Stream`
  handling, §4.3).
- **Exit criteria**: a manual streamed chat turn renders token-by-token in
  the TUI exactly like an Ollama/OpenAI turn today (no visible difference
  in rendering behavior — only the source provider differs); cancelling a
  stream mid-generation (session switch, `Esc`, etc.) doesn't leave the
  worker thread stuck mid-decode for the next request.

### Phase 3 — Sampling, context, chat templates

- **Deliverables**: full `LlamaCppProviderConfig` sampling fields
  (temperature/top_p/top_k/repeat_penalty/seed) wired into the sampler
  chain; embedded-vs-override chat template resolution (§4.6); `n_ctx`
  reporting via `Provider::context_window()`; stop-sequence matching against
  growing detokenized output; `max_tokens`/`n_predict` enforcement.
- **Files**: `src/llm/provider/llama_cpp.rs`.
- **Exit criteria**: a model with a known embedded GGUF chat template
  produces well-formed turns without a configured `chat_template` override;
  a model whose embedded template is missing/broken is usable once
  `chat_template` is set manually; `context_window()` returns exactly the
  `n_ctx` the context was created with, never a stale/assumed value (same
  invariant `OllamaProvider::context_window()` already enforces per
  `ollama.rs:756-772`).

### Phase 4 — Tool calling

- **Deliverables**: extract `OllamaProvider`'s private tool-call recovery
  functions (`maybe_tool_call_json`, `tool_call_from_content`,
  `fenced_json_blocks`, `parse_tool_call_object` — currently
  `ollama.rs:807-921`; this deliberately excludes the neighboring
  `collect_tool_calls`/`stop_reason_for` at `ollama.rs:785-799`, which stay
  Ollama-specific stream-accumulation helpers, not recovery logic) into a
  new shared module; re-point `OllamaProvider`
  at it (behavior-preserving refactor); wire the same module into
  `LlamaCppProvider`, plus a system-prompt template describing the offered
  tools as JSON functions (§4.7).
- **Files**: new `src/llm/provider/tool_call_recovery.rs`;
  `src/llm/provider/ollama.rs` (delete the extracted functions, call the
  shared module — its ~15 existing unit tests for this logic, e.g.
  `tool_call_printed_as_content_is_recovered`,
  `fenced_call_in_prose_becomes_a_tool_use_block`, move with it and must
  still pass unmodified); `src/llm/provider/mod.rs` (module registration);
  `src/llm/provider/llama_cpp.rs` (tool-prompt construction + recovery call).
- **Exit criteria**: every existing Ollama tool-recovery test still passes
  post-extraction with zero behavior change (this is the regression bar,
  not just "tests exist"); a new equivalent suite for `LlamaCppProvider`
  covers at least the same cases (recovered call, fenced-in-prose call,
  first-of-several, non-tool JSON not recovered, prose never mistaken for a
  call); a manual test drives a real local model through a multi-tool
  agentic turn (e.g. `bash` + `read_file`) end to end.

### Phase 4b — Grammar-constrained tool calling (`llguidance`, optional)

Deferred, additive, and gated on Phase 0 confirming the feature's actual
Rust API shape (open question §10.1) — not part of the Phases 0-5
mergeable slice (§4.7).

- **Deliverables**: `llama-cpp-llguidance` Cargo feature (§4.1); mapping
  from `Tool.input_schema` (JSON Schema, already generic in
  `provider/types.rs`) to an `llguidance`-consumable grammar constraining
  decoding to valid-schema output; wired as a pre-check *before* the
  Phase 4 recovery heuristic runs, which stays the always-on fallback
  (§4.7 — grammar constraints guarantee syntax, not budget-safe
  completion, so recovery remains meaningful even with this on).
- **Files**: `Cargo.toml` (new feature); `src/llm/provider/llama_cpp.rs`
  (grammar construction + sampler wiring, behind
  `#[cfg(feature = "llama-cpp-llguidance")]`).
- **Exit criteria**: with the feature compiled in, a manual multi-tool
  agentic turn against a model previously observed to print malformed
  tool-call JSON (if one was identified during Phase 4's manual testing)
  no longer needs the recovery heuristic to succeed — verified by adding a
  temporary log line confirming the heuristic path wasn't hit; without the
  feature compiled in, behavior is byte-for-byte identical to Phase 4
  (recovery-only), confirming this is genuinely additive, not a silent
  behavior change gated on a feature flag.

### Phase 5 — Performance metrics

- **Deliverables**: `PerfMetrics` populated from `llama.cpp`'s
  load/prefill/eval counters (§4.8), flowing through the existing
  `LLMResponse → AgentResponse → DisplayMessage → Session` pipeline
  unchanged.
- **Files**: `src/llm/provider/llama_cpp.rs` only, plus a one-line doc
  comment fix at `src/llm/agent/service.rs:1766-1767` (`AgentResponse
  .perf_metrics`'s doc currently reads "if the provider exposes them
  (currently only the native Ollama provider)" — no longer accurate once
  this phase lands). **No migration file** — confirmed in §4.8 that
  `messages.provider_name`/`perf_metrics_json` and `sessions.provider`
  already exist and are provider-agnostic.
- **Exit criteria**: TUI header shows the `llama-cpp` provider badge and
  tok/s segment on a real local generation, with no code changes needed in
  `tui/render.rs` beyond a new `provider_icon()` match arm; a fresh session
  vs. a session reloaded from DB both show correct perf data (round-trips
  through `perf_metrics_json` correctly).

*Phases 0–5 are the minimum viable, mergeable slice: a working, tool-capable,
tested provider with performance metrics, reachable via config and the
factory, with no TUI-specific code beyond the one-line provider-icon
addition. Phase 4b (`llguidance`) is explicitly outside this slice — an
optional reliability upgrade on top of it, not a dependency of it.*

### Phase 6 — Model management (file-based)

- **Deliverables**: `list_local_models`/`download_model`/`delete_model`
  (§8); `hf:org/repo/file.gguf` shorthand resolution plus its SHA-256
  metadata lookup (§8.2, §4.11 point 2); CLI subcommand (§8.1).
- **Files**: new `src/llm/provider/llama_cpp_models.rs`; `src/cli/mod.rs`
  (`Commands::LlamaCpp`, `LlamaCppCommands`, `cmd_llama_cpp`,
  `llama_cpp_models_dir` — mirroring the `Ollama`/`cmd_ollama`/
  `ollama_host` block at `cli/mod.rs:225-234,1090-1200` line-for-line in
  structure).
- **Exit criteria**: `crustly llama-cpp list` on an empty `models_dir`
  prints the same style of "(none) — pull one with…" hint `crustly ollama
  list` does; `crustly llama-cpp pull hf:org/repo/file.gguf` downloads with
  a visible terminal progress bar, checks the file's SHA-256 against
  Hugging Face's published hash when available (§8.2) and fails loudly with
  both hashes shown on a mismatch, and the file appears in a subsequent
  `list`; a source with no published hash prints the "no integrity hash
  available" warning rather than downloading silently (§4.11); `crustly
  llama-cpp rm <file>` requires confirmation and removes the file; without
  `--features llama-cpp`, all three subcommands print the same "rebuild
  with `--features llama-cpp`" message `cmd_ollama`'s disabled path does,
  not a panic or a silent no-op.

### Phase 7 — TUI integration

- **Deliverables**: new model picker/download dialog per §7 point 4 option
  (a) — new keybinding, `AppMode::LlamaCppModelPicker`, new
  `src/tui/llama_cpp_download.rs`; Model Info panel extended for
  `llama.cpp`-specific fields (GPU layers, quantization, `n_ctx`);
  status-bar error surfaces for model-not-found/OOM/incompatible-GGUF.
  Provider badge and per-message perf footer need **no dedicated
  implementation** — already generic since Phase 5.
- **Files**: `src/tui/events.rs` (new `keys::is_llama_cpp_models` + new
  `AppMode` variant); new `src/tui/llama_cpp_download.rs` (mirrors
  `src/tui/ollama_download.rs`'s shape: suggestions/filtering,
  `DownloadProgress`-driven overlay); `src/tui/app.rs` (new
  `llama_cpp_download_task` field, mirroring `model_download_task` at
  `app.rs:153`; new `handle_*_key` dispatch arm alongside the existing
  `AppMode::ModelDownload`/`AppMode::ProviderSwitch` arms at `app.rs:846-847`);
  `src/tui/components/dialogs/mod.rs` (new render function, same
  `Clear`+`Block` overlay family as `render_auto_exec_progress`);
  `src/tui/render.rs` (one new `provider_icon()` match arm, Model Info
  panel extension).
- **Exit criteria**: opening the new dialog, picking a local `.gguf`, and
  swapping to it shows the "Loading model…" blocking state (§4.5) rather
  than an instant swap; downloading a new model shows live byte progress
  and the model is immediately selectable afterward; none of the existing
  Ollama dialogs' snapshot/behavior tests regress (constraint §3.2 — this
  is the actual verification, not an assumption).

### Phase 8 — GPU acceleration

Scope gated on open question §10.8 (which backends launch first) — the
deliverables below cover `cuda`/`metal` as the recommended first pair
(§10.8); `rocm`/`opencl`/`mkl` are real, equally-supported upstream
features (§4.1) that can follow the same pattern if/when prioritized, not
a fundamentally different effort.

- **Deliverables**: `llama-cpp-cuda`/`llama-cpp-metal`/`llama-cpp-vulkan`
  Cargo features, plus `llama-cpp-rocm`/`llama-cpp-opencl`/`llama-cpp-mkl`
  if in scope per §10.8 (§4.1); `n_gpu_layers` wired through to context
  creation; documentation of hardware requirements and approximate VRAM
  usage per quantization level (e.g. Q4_K_M vs Q8_0 vs F16) for common
  model sizes.
- **Files**: `Cargo.toml`; `src/llm/provider/llama_cpp.rs` (GPU-layers
  param, warning log when `n_gpu_layers > 0` but no GPU feature compiled
  in); `docs/guides/LLAMA_CPP_GUIDE.md` (started here, finished in Phase 10).
- **Exit criteria**: a CUDA-enabled build measurably offloads layers (VRAM
  usage visible via `nvidia-smi` during a real generation) and shows a
  materially higher tok/s than the CPU-only build for the same model in the
  perf-metrics footer (Phase 5) — the two builds' *own* metrics are the
  proof, no separate benchmarking harness needed.

### Phase 9 — Idle unload & model-swap hardening

- **Deliverables**: `idle_unload_secs` timer (§4.5) on the worker thread;
  worker-thread panic isolation via `catch_unwind` (§9) if not already done
  in Phase 1; resumable downloads via HTTP `Range` requests (§8) if
  prioritized by user feedback from Phase 6.
- **Files**: `src/llm/provider/llama_cpp.rs`;
  `src/llm/provider/llama_cpp_models.rs` (resumable download, optional).
- **Exit criteria**: a provider left idle past `idle_unload_secs` frees its
  RAM/VRAM (observable via process memory / `nvidia-smi`) and the next
  request transparently reloads and succeeds, paying the load cost once; an
  injected panic inside a `complete()`/`stream()` call (test harness, not
  manual) is caught, reported as an `Err` to that one caller, and the
  worker thread demonstrably serves the *next* request rather than hanging
  or the process crashing.

### Phase 10 — Documentation & rollout

- **Deliverables**: README section (mirroring the Ollama section's
  "two paths" clarity, extended to name this a *third* local-inference
  path distinct from both Ollama modes); `config.toml.example`
  `[providers.llama_cpp]` block; `docs/guides/LLAMA_CPP_GUIDE.md` completed;
  ADR recording the worker-thread-per-model decision from §4.4.
- **Files**: `README.md` (including the process-isolation/trust posture
  from §4.11, stated plainly rather than implied); `config.toml.example`;
  `docs/guides/LLAMA_CPP_GUIDE.md`; new
  `docs/architecture/decisions/000X-llama-cpp-in-process-worker-thread.md`
  (next free number after `0004-plan-mode-read-only-with-approval-gating.md`
  — confirm the actual next number at implementation time), following the
  format of `0002-sqlx-over-rusqlite.md`/`0003-crabrace-provider-registry.md`
  (Context/Decision/Consequences), explicitly recording the §7 point 4
  keybinding/dialog decision as a Consequence, and — like ADR `0003`
  pointing to `docs/guides/CRABRACE_INTEGRATION.md` for wire-protocol detail
  — pointing to **both** `llm-file-gguf-support.md` (the original
  benefit/cost/Go-No-Go analysis) and this plan (the execution detail) for
  anyone asking "why does this exist and why does it work this way" later.
- **Exit criteria**: a new contributor can go from a clean checkout to a
  running local `llama.cpp` chat session using only the README section, no
  tribal knowledge; the ADR is linked from `docs/architecture/decisions/README.md`
  alongside the existing four, and itself links back to both source
  documents rather than restating their analysis.

Phases 6–10 are additive UX/ops polish and can ship incrementally after
Phase 5 without blocking each other or requiring a specific order beyond
Phase 6 (file management) preceding Phase 7 (the TUI dialog that calls it).
