# `llama-cpp-2` Integration Plan

Status: **Planning — not started.** No code has been written yet; this
document is the design and phasing reference for the implementation.
Branch: `claude/llama-cpp-2-integration-mfirvd`
Dependency: [`llama-cpp-2`](https://crates.io/crates/llama-cpp-2) (Rust
bindings over `llama.cpp` via `llama-cpp-sys-2`)

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
| Build complexity | None (pure Rust HTTP client) | Significant — native C++ compilation (Section 3.4) |

In short: Ollama is the better choice for most users today and remains the
recommended default. `llama-cpp-2` is for users who specifically want
zero-server local inference, already manage their own GGUF files, or want
tighter control over GPU offload than Ollama exposes. Given the build-cost
tradeoff in the row above, this is deliberately **not** proposed for the
`default` or even `all-llm` feature set — see Section 3.4.

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

## 4. Architecture

### 4.1 Dependencies (`Cargo.toml`)

```toml
[dependencies]
llama-cpp-2 = { version = "0.1", optional = true, default-features = false }

[features]
llama-cpp = ["dep:llama-cpp-2"]
llama-cpp-cuda = ["llama-cpp", "llama-cpp-2/cuda"]
llama-cpp-metal = ["llama-cpp", "llama-cpp-2/metal"]
llama-cpp-vulkan = ["llama-cpp", "llama-cpp-2/vulkan"]
# NOTE: deliberately NOT added to `all-llm` — see §3.4.
```

Exact version, feature names on the upstream crate (`cuda`/`metal`/`vulkan`
vs. whatever `llama-cpp-2` currently calls them), and MSRV impact must be
confirmed against the version on crates.io at implementation time — this is
Phase 0's job, not assumed here.

### 4.2 Build requirements (must be documented, not hidden)

Building with `--features llama-cpp` additionally requires, on the build
machine:

- A C and C++ toolchain (`cc`, `cmake` ≥ 3.14 — `llama-cpp-sys-2` builds
  `llama.cpp` from its vendored/pinned source via `cmake`).
- For `llama-cpp-cuda`: the CUDA toolkit matching the GPU driver.
- For `llama-cpp-metal`: Xcode command-line tools (macOS only; this feature
  is a no-op / should be excluded from non-Apple targets).
- For `llama-cpp-vulkan`: the Vulkan SDK.

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
```

`Provider::complete()`/`stream()` become thin: build the `LLMRequest` into
an `InferenceJob`, send it over the channel, and `.await` the response (a
`oneshot` for `complete`, forwarding the `mpsc` receiver as the returned
`ProviderStream` for `stream`). All actual `llama.cpp` FFI calls happen only
on the worker thread — this is the load-bearing invariant of the whole
design (§4.4 explains why).

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

**Stretch goal, later phase, not v1**: grammar-constrained decoding via
GBNF (`llama.cpp`'s grammar sampler) built from the offered tool's JSON
Schema, which would make malformed tool-call JSON structurally impossible
rather than merely recovered/retried. This needs either a JSON-Schema→GBNF
converter (porting the relevant slice of `llama.cpp`'s own
`json-schema-to-grammar` logic) or accepting a coarser "valid JSON object"
grammar and keeping the existing strict `parse_tool_call_object` validation
on top. Flagged as an open question (§10) rather than committed to a phase
number, since it depends on what the pinned `llama-cpp-2` version actually
exposes for grammar sampling.

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

### 4.9 Vision support

Multimodal (LLaVA-style, via a companion `mmproj` GGUF file) is **out of
scope for v1**. `llama-cpp-2`'s multimodal support is less mature/stable
than its text-only path as of this writing, and Crustly's existing
`ContentBlock::Image` handling would need a second GGUF file's worth of new
config (`mmproj_path`) and a different decode path (image embedding before
text tokens). `supports_vision()` returns `false` unconditionally for v1;
revisit as a follow-up plan once the text-only path is stable, not bundled
into this one.

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
   a `provider_icon()` entry (e.g. `🦙+` or `⚙️` for `llama-cpp`) — the
   pipeline is already generic over any provider that sets
   `perf_metrics`/`provider_name` (§4.8).
2. **Per-message perf footer**: automatic, same reason.
3. **Model Info panel** (`Ctrl+O`, already built per the Ollama plan's
   follow-up): extend to show `llama.cpp`-specific fields (GPU layers
   offloaded, quantization type parsed from the GGUF filename/metadata,
   context size) when the active provider is `llama-cpp`.
4. **Model picker / "download" dialog** (`Ctrl+D`, reusing
   `src/tui/ollama_download.rs`'s pattern): for `llama.cpp` this is
   necessarily different from Ollama's `pull <name>` — there is no
   `llama.cpp` model registry API. Two sub-flows:
   - **Local pick**: list `.gguf` files already present under
     `providers.llama_cpp.models_dir`, let the user select one to become
     the active `model_path` (triggers the model-swap loading flow, §4.5,
     with a blocking "Loading model…" progress state — explicitly not
     instant like Ollama's swap).
   - **Download by URL/HF repo** (§8): a text field for a direct `.gguf`
     URL or a `hf:org/repo/file.gguf`-style shorthand, downloaded into
     `models_dir` with a byte-progress bar (reusing the same `Clear` +
     `Block` overlay family as the Ollama pull dialog, but driven by
     `reqwest`'s streaming byte progress instead of Ollama's layer-based
     pull events — different progress unit, same UI shell).
5. **Status bar**: `llama.cpp`-specific error surfaces — model file not
   found, out-of-memory on load (common with GPU offload misconfigured),
   GGUF version incompatible with the compiled `llama.cpp` — mapped to
   actionable messages (§4.6/§9 error mapping), not raw FFI error strings.

## 8. Model management (no server, so this is file management)

New optional module `src/llm/provider/llama_cpp_models.rs` (feature-gated),
providing what Ollama's `/api/tags`/`/api/pull`/`/api/delete` give for free
from a server, implemented here as local filesystem + HTTP download
operations:

- `list_local_models(models_dir) -> Vec<LocalGgufModel>` — scans
  `models_dir` for `*.gguf`, parsing size on disk and, where feasible,
  quantization/parameter-count hints from GGUF header metadata (a cheap
  partial read, not a full model load) or the filename convention
  (`*-q4_k_m.gguf`, `*-Q8_0.gguf`) as a fallback.
- `download_model(url_or_hf_ref, models_dir, on_progress) -> PathBuf` —
  streams the file via the crate's existing `reqwest` client (no new HTTP
  dependency), reporting `(bytes_downloaded, total_bytes)` progress through
  a channel, same shape as Ollama's `PullProgressEvent` so the TUI overlay
  code can share structure. Supports resuming an interrupted download via
  `Range` requests if the partial file is still present (nice-to-have, not
  a Phase-1 requirement).
- `delete_model(path) -> Result<()>` — simple file removal, with a
  confirmation step in both the CLI and TUI (deleting a multi-GB file is not
  reversible).

CLI subcommand `crustly llama-cpp <list|pull|rm>`, mirroring
`crustly ollama <list|pull|rm|show>` (`src/cli/mod.rs`), feature-gated
identically.

**`hf:org/repo/file.gguf` shorthand**: resolves to
`https://huggingface.co/org/repo/resolve/main/file.gguf`. This is
string-substitution only — no HuggingFace API client dependency needed for
v1. Document that gated/private HF repos aren't supported without a token
(`HF_TOKEN` env var forwarded as an `Authorization` header is a reasonable
Phase-6 addition if requested; not assumed here).

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
   template application, grammar/GBNF sampling, and performance-counter
   access. This plan describes the *shape* of the integration; Phase 0 must
   confirm these calls exist as described (or adjust) against the version
   actually pinned.
2. **One provider instance per model, or one provider managing several
   `model_path`s with hot-swap?** This plan assumes the former (simpler,
   matches `OllamaProviderConfig`'s single-`host` shape) — multiple local
   GGUF models would mean multiple `[[providers.llama_cpp]]`-style entries
   or repeated single-model configs selected via the existing model-switch
   UX (§7 point 4). Needs a decision before §5's config shape is finalized
   if multi-model-without-reload turns out to be a common ask.
3. **GBNF grammar-constrained tool calling** (§4.7 stretch goal) — commit to
   a phase number, or leave as prompt-based recovery indefinitely (matching
   what Ollama's fallback path already does for non-conforming templates)?
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

## 13. Phasing

- **Phase 0 — Feasibility spike** (no user-facing code): pin an exact
  `llama-cpp-2` version, build it in isolation against this project's
  `Cargo.lock`, confirm no dependency conflicts, measure clean-build time
  and binary size delta, confirm which of chat-template application /
  grammar sampling / perf counters (§10.1) are actually exposed by the
  pinned version. **Go/no-go gate** for the rest of the plan — if the pinned
  version is missing something load-bearing (e.g. no perf counters), later
  phases are adjusted before committing to their design above.
- **Phase 1 — MVP provider (CPU only, non-streaming)**: `llama-cpp` feature
  flag, `LlamaCppProvider` with the worker-thread architecture (§4.3–4.4),
  `complete()` only, config (§5), factory wiring (§6), error mapping (§9,
  minus the panic-catch hardening which can land with it or right after).
  Testable in isolation; no TUI changes yet.
- **Phase 2 — Streaming**: token-by-token generation through the worker
  thread into a `ProviderStream`, matching `StreamEvent` sequencing used by
  every other provider's `stream()`.
- **Phase 3 — Sampling, context, chat templates**: full
  `LlamaCppProviderConfig` sampling fields, embedded/override chat template
  handling, `n_ctx` reporting, stop sequences, seed.
- **Phase 4 — Tool calling**: extract the shared recovery module (§4.7),
  wire it into both `OllamaProvider` (refactor, behavior-preserving) and
  `LlamaCppProvider` (new).
- **Phase 5 — Performance metrics**: wire `PerfMetrics` from `llama.cpp`'s
  counters (§4.8) — this is expected to be quick, since the TUI plumbing
  already exists from Ollama's rollout.
- **Phase 6 — Model management**: `llama_cpp_models.rs` (list/download/
  delete), CLI subcommand (§8).
- **Phase 7 — TUI integration**: model picker/download dialog (§7 point 4),
  Model Info panel extension, status-bar error surfaces. Provider badge and
  perf footer need no dedicated work (already generic, §4.8/§7 points 1–2).
- **Phase 8 — GPU acceleration**: `llama-cpp-cuda`/`-metal`/`-vulkan`
  features, `n_gpu_layers` wiring, documentation of hardware requirements
  and expected VRAM usage per quantization level.
- **Phase 9 — Idle unload & model-swap hardening**: `idle_unload_secs`
  timer (§4.5), worker-thread panic isolation (§9) if not already done in
  Phase 1, resumable downloads (§8) if prioritized.
- **Phase 10 — Documentation & rollout**: README section (mirroring the
  Ollama section's "two paths" clarity — here, clarify this is a *third*
  path, distinct from both Ollama modes), `config.toml.example` block,
  `docs/guides/LLAMA_CPP_GUIDE.md`, and an ADR
  (`docs/architecture/decisions/000X-llama-cpp-in-process-worker-thread.md`)
  documenting the worker-thread-per-model decision from §4.4 — this is
  exactly the kind of cross-cutting architectural choice the existing ADR
  process (see `0002-sqlx-over-rusqlite.md`, `0003-crabrace-provider-registry.md`)
  is for.

Phases 0–5 are the minimum viable, mergeable slice (a working, testable
provider with no TUI changes). Phases 6–10 are additive UX/ops polish and
can ship incrementally after Phase 5 without blocking each other.
