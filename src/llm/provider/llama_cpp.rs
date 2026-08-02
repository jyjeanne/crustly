//! In-process `llama.cpp` provider (loads a local `.gguf` file directly).
//!
//! Unlike every other provider in this module, this one is not an HTTP
//! client - it loads model weights and runs inference inside this process,
//! via the `llama-cpp-2` crate (FFI bindings to `llama.cpp`). See
//! `llama-cpp-2-integration-plan.md` for the full design; this file
//! implements Phases 1-4 (`complete()`, `stream()`, full sampling/context
//! reporting, and tool-call recovery via the shared
//! `tool_call_recovery` module). Grammar-constrained tool calling
//! (`llguidance`, Phase 4b) is not implemented yet.
//!
//! ## Threading model
//!
//! `LlamaModel` is `Send + Sync` (the upstream crate marks it so - read-only
//! weights), but `LlamaContext` and `LlamaBackend` are not: they are not
//! safe to move across threads or share concurrently (confirmed by reading
//! `llama-cpp-2` 0.1.153's source - neither type has an `unsafe impl
//! Send`/`Sync`). All three are therefore created and used exclusively on a
//! single dedicated OS thread ("the worker"), following the same
//! sync-thread-bridged-to-tokio pattern already used by
//! `crate::app::start_file_watcher` (`src/app/mod.rs`) for another
//! non-async library (`notify::Watcher`): a `tokio::sync::mpsc` channel
//! carries requests in, the worker thread drains it with `blocking_recv()`,
//! and responses go back out through a `oneshot` channel per request.
//!
//! Model loading itself also happens on the worker thread (not the caller's
//! thread), for the same reason - `LlamaCppProvider::new()` blocks
//! (via a `oneshot`) until the worker confirms the model loaded or reports
//! why it didn't, so callers (the factory) get a normal `Result` without
//! any FFI call ever happening off the worker thread.

use super::error::{ProviderError, Result};
use super::r#trait::{Provider, ProviderStream};
use super::tool_call_recovery::{maybe_tool_call_json, tool_call_from_content};
use super::types::*;
use async_trait::async_trait;
use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::context::LlamaContext;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::{AddBos, LlamaChatMessage, LlamaChatTemplate, LlamaModel};
use llama_cpp_2::sampling::LlamaSampler;
use llama_cpp_2::token::LlamaToken;
use std::num::NonZeroU32;
use std::path::PathBuf;
use tokio::sync::{mpsc, oneshot};

/// Default generation cap when neither the request nor config sets one -
/// deliberately conservative so a runaway generation on a small `n_ctx`
/// can't silently consume the whole context window.
const DEFAULT_MAX_TOKENS: u32 = 2_048;

/// Sampling defaults applied when a request doesn't specify its own -
/// mirrors `OllamaProvider`'s rationale (`ollama.rs`): a local model rarely
/// behaves well on a provider that sends nothing and lets some other
/// implicit default apply.
#[derive(Debug, Clone)]
struct SamplingDefaults {
    temperature: f32,
    top_p: f32,
    top_k: i32,
    repeat_penalty: f32,
}

impl Default for SamplingDefaults {
    fn default() -> Self {
        Self {
            temperature: 0.8,
            top_p: 0.95,
            top_k: 40,
            repeat_penalty: 1.1,
        }
    }
}

/// One request submitted to the worker thread.
enum InferenceJob {
    /// A non-streaming request: the full response is sent back once,
    /// through a `oneshot`.
    Complete {
        request: LLMRequest,
        respond_to: oneshot::Sender<Result<LLMResponse>>,
    },
    /// A streaming request: `StreamEvent`s are pushed incrementally through
    /// `events_tx` as they're generated - `stream()` returns the receiver
    /// end (wrapped as a `Stream`) immediately, without waiting for
    /// generation to start or finish, unlike a buffer-then-replay approach.
    Stream {
        request: LLMRequest,
        events_tx: mpsc::UnboundedSender<Result<StreamEvent>>,
    },
}

/// In-process `llama.cpp` provider. Cloning shares the same worker thread
/// and loaded model (the `mpsc::UnboundedSender` is cheaply cloneable) -
/// it does not load a second copy.
#[derive(Clone)]
pub struct LlamaCppProvider {
    job_tx: mpsc::UnboundedSender<InferenceJob>,
    model_path: PathBuf,
    display_name: String,
    n_ctx: u32,
}

impl LlamaCppProvider {
    /// Load `config.model_path` and spawn the worker thread. Blocks (via a
    /// `oneshot`) until the model has finished loading (or failed to), so
    /// the returned `Result` is meaningful - callers don't get a "success"
    /// that silently can't serve any request.
    pub fn new(config: &crate::config::LlamaCppProviderConfig) -> Result<Self> {
        if !config.model_path.exists() {
            return Err(ProviderError::ModelNotFound(format!(
                "llama.cpp model file not found: {}. Check `providers.llama_cpp.model_path` \
                 in config.toml.",
                config.model_path.display()
            )));
        }

        let display_name = config.display_name.clone().unwrap_or_else(|| {
            config
                .model_path
                .file_stem()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_else(|| "llama-cpp-model".to_string())
        });

        let (job_tx, job_rx) = mpsc::unbounded_channel::<InferenceJob>();
        // Carries back the context's *actually resolved* n_ctx, not just the
        // requested one - `NonZeroU32::new(0)` (a user setting `n_ctx = 0` to
        // mean "use the model's trained default") makes llama.cpp pick its
        // own value, which `context_window()` must report accurately rather
        // than echoing back a config value that was never really applied.
        let (ready_tx, ready_rx) = std::sync::mpsc::channel::<std::result::Result<u32, String>>();

        let model_path = config.model_path.clone();
        let n_ctx = config.n_ctx;
        let n_gpu_layers = config.n_gpu_layers;
        let n_threads = config
            .n_threads
            .unwrap_or_else(|| std::thread::available_parallelism().map_or(4, |n| n.get() as u32));
        let chat_template_override = config.chat_template.clone();
        let sampling_defaults = SamplingDefaults {
            temperature: config.temperature.unwrap_or(0.8),
            top_p: config.top_p.unwrap_or(0.95),
            top_k: config.top_k.map(|k| k as i32).unwrap_or(40),
            repeat_penalty: config.repeat_penalty.unwrap_or(1.1),
        };
        let seed = config.seed;
        let idle_unload_secs = config.idle_unload_secs;

        std::thread::spawn(move || {
            worker_loop(WorkerInit {
                model_path,
                n_ctx,
                n_gpu_layers,
                n_threads,
                chat_template_override,
                sampling_defaults,
                seed,
                idle_unload_secs,
                job_rx,
                ready_tx,
            });
        });

        match ready_rx.recv() {
            Ok(Ok(actual_n_ctx)) => Ok(Self {
                job_tx,
                model_path: config.model_path.clone(),
                display_name,
                n_ctx: actual_n_ctx,
            }),
            Ok(Err(msg)) => Err(ProviderError::Internal(format!(
                "failed to load llama.cpp model '{}': {msg}",
                config.model_path.display()
            ))),
            Err(_) => Err(ProviderError::Internal(
                "llama.cpp worker thread exited before reporting load status".to_string(),
            )),
        }
    }
}

struct WorkerInit {
    model_path: PathBuf,
    n_ctx: u32,
    n_gpu_layers: u32,
    n_threads: u32,
    chat_template_override: Option<String>,
    sampling_defaults: SamplingDefaults,
    seed: Option<u32>,
    /// Auto-unload the model after this many idle seconds (§4.5); `None`
    /// never unloads. Reloaded lazily on the next job after an unload.
    idle_unload_secs: Option<u64>,
    job_rx: mpsc::UnboundedReceiver<InferenceJob>,
    ready_tx: std::sync::mpsc::Sender<std::result::Result<u32, String>>,
}

/// How often the idle-unload poll loop wakes up to check the clock while a
/// model is loaded and `idle_unload_secs` is configured. A fixed short
/// sleep rather than a real timeout-capable receive, because
/// `tokio::sync::mpsc::UnboundedReceiver` has no `blocking_recv` variant
/// with a timeout - negligible overhead at this interval against a
/// threshold measured in minutes.
const IDLE_POLL_INTERVAL: std::time::Duration = std::time::Duration::from_secs(2);

/// The worker thread body: loads the model, then serially drains
/// `InferenceJob`s until the sender is dropped. Every `llama.cpp` FFI call
/// in this provider happens inside this function or the functions it calls
/// - never on the async caller's thread. See the module doc for why.
///
/// Structured as an outer "load, then serve" loop rather than a single
/// load followed by one serve loop, so an idle-unload can drop
/// `backend`/`model`/`context` (by simply letting one iteration's locals go
/// out of scope) and the next iteration can reload them - `LlamaContext`
/// borrows from `LlamaModel` (see the module doc's Send/Sync note), so the
/// two can only coexist as sibling locals in the same stack frame, not as
/// fields of a struct stored across iterations without a self-referential
/// type. Only the *first* iteration's load result is reported through
/// `ready_tx` (`new()`'s contract: fail fast on a bad model file at
/// startup); later reloads after an idle-unload log instead.
fn worker_loop(init: WorkerInit) {
    let WorkerInit {
        model_path,
        n_ctx,
        n_gpu_layers,
        n_threads,
        chat_template_override,
        sampling_defaults,
        seed,
        idle_unload_secs,
        mut job_rx,
        ready_tx,
    } = init;
    let mut ready_tx = Some(ready_tx);
    // The job that woke us up from a post-idle-unload wait, to be handled
    // immediately once the reload below completes - `None` on the very
    // first (eager) load, where nothing has been received yet.
    let mut carry_over_job: Option<InferenceJob> = None;

    loop {
        let backend = match LlamaBackend::init() {
            Ok(b) => b,
            Err(e) => {
                if let Some(tx) = ready_tx.take() {
                    let _ = tx.send(Err(format!("failed to init llama.cpp backend: {e:?}")));
                }
                return;
            }
        };

        let model_params = LlamaModelParams::default().with_n_gpu_layers(n_gpu_layers);
        let model = match LlamaModel::load_from_file(&backend, &model_path, &model_params) {
            Ok(m) => m,
            Err(e) => {
                if let Some(tx) = ready_tx.take() {
                    let _ = tx.send(Err(format!("failed to load GGUF file: {e:?}")));
                }
                return;
            }
        };

        let context_params = LlamaContextParams::default()
            .with_n_ctx(NonZeroU32::new(n_ctx))
            .with_n_threads(n_threads as i32)
            .with_n_threads_batch(n_threads as i32);
        let mut context = match model.new_context(&backend, context_params) {
            Ok(c) => c,
            Err(e) => {
                if let Some(tx) = ready_tx.take() {
                    let _ = tx.send(Err(format!("failed to create llama.cpp context: {e:?}")));
                }
                return;
            }
        };
        // The context's *actually resolved* n_ctx - may differ from the
        // requested `n_ctx` (e.g. `n_ctx = 0` asks llama.cpp for the
        // model's own trained default). This, not the config value, is
        // what `Provider::context_window()` must report.
        let actual_n_ctx = context.n_ctx();

        // Resolve the chat template once per load: an explicit config
        // override always wins; otherwise use the model's own embedded
        // GGUF template if present. A model with neither gets a minimal
        // manual fallback assembled per-request in `build_prompt` - not
        // stored here since it isn't a `LlamaChatTemplate`.
        let chat_template: Option<LlamaChatTemplate> = chat_template_override
            .as_deref()
            .and_then(|t| LlamaChatTemplate::new(t).ok())
            .or_else(|| model.chat_template(None).ok());

        match ready_tx.take() {
            Some(tx) => {
                // First load - tell `new()` it can return `Ok`.
                if tx.send(Ok(actual_n_ctx)).is_err() {
                    // The caller gave up waiting (e.g. timed out, or the
                    // process is shutting down) - nothing to serve.
                    return;
                }
            }
            None => {
                tracing::info!(
                    "llama.cpp: model reloaded after an idle-unload ({})",
                    model_path.display()
                );
            }
        }

        let mut last_activity = std::time::Instant::now();
        let mut shutting_down = false;

        // The job that triggered this reload (if any) is handled first,
        // before entering the wait-for-next-job loop below.
        if let Some(job) = carry_over_job.take() {
            dispatch_job(&model, &mut context, &chat_template, &sampling_defaults, seed, job);
            last_activity = std::time::Instant::now();
        }

        loop {
            let job = if let Some(idle_secs) = idle_unload_secs {
                match job_rx.try_recv() {
                    Ok(job) => job,
                    Err(mpsc::error::TryRecvError::Empty) => {
                        if last_activity.elapsed() >= std::time::Duration::from_secs(idle_secs) {
                            tracing::info!(
                                "llama.cpp: unloading idle model after {idle_secs}s ({})",
                                model_path.display()
                            );
                            break;
                        }
                        std::thread::sleep(IDLE_POLL_INTERVAL);
                        continue;
                    }
                    Err(mpsc::error::TryRecvError::Disconnected) => {
                        shutting_down = true;
                        break;
                    }
                }
            } else {
                match job_rx.blocking_recv() {
                    Some(job) => job,
                    None => {
                        shutting_down = true;
                        break;
                    }
                }
            };

            dispatch_job(&model, &mut context, &chat_template, &sampling_defaults, seed, job);
            last_activity = std::time::Instant::now();
        }

        if shutting_down {
            return;
        }

        // Idle-unload fired (not shutdown): `backend`/`model`/`context`
        // drop here as this loop iteration ends. Block for real (no
        // polling, zero CPU) until the next job arrives before paying the
        // reload cost - reloading speculatively before there's a job to
        // serve would defeat the point of unloading.
        match job_rx.blocking_recv() {
            Some(job) => carry_over_job = Some(job),
            None => return,
        }
    }
}

/// Dispatch one job against the currently loaded model/context, with panic
/// isolation so an FFI-adjacent panic can't take the worker thread (and
/// therefore every future request) down with it.
fn dispatch_job(
    model: &LlamaModel,
    context: &mut LlamaContext<'_>,
    chat_template: &Option<LlamaChatTemplate>,
    sampling_defaults: &SamplingDefaults,
    seed: Option<u32>,
    job: InferenceJob,
) {
    match job {
        InferenceJob::Complete {
            request,
            respond_to,
        } => {
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                run_complete(model, context, chat_template, sampling_defaults, seed, request)
            }))
            .unwrap_or_else(|payload| Err(panic_to_provider_error(&payload)));
            let _ = respond_to.send(result);
        }
        InferenceJob::Stream { request, events_tx } => {
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                run_stream(
                    model,
                    context,
                    chat_template,
                    sampling_defaults,
                    seed,
                    request,
                    &events_tx,
                )
            }));
            if let Err(payload) = result {
                // A panic anywhere in run_stream means no terminal event
                // (ContentBlockStop/MessageStop) was sent - tell the
                // consumer why the stream just stops, rather than leaving
                // it hanging with no explanation.
                let _ = events_tx.send(Err(panic_to_provider_error(&payload)));
            }
        }
    }
}

/// Shared panic->error conversion + logging for both job kinds.
fn panic_to_provider_error(payload: &(dyn std::any::Any + Send)) -> ProviderError {
    let msg = panic_message(payload);
    tracing::error!("llama.cpp worker panicked during a request: {msg}");
    ProviderError::Internal(format!("llama.cpp inference panicked: {msg}"))
}

/// Best-effort extraction of a human-readable message from a caught panic
/// payload (`&str` and `String` cover the overwhelming majority of panics;
/// anything else falls back to a generic message rather than failing to
/// report at all).
fn panic_message(payload: &(dyn std::any::Any + Send)) -> String {
    if let Some(s) = payload.downcast_ref::<&str>() {
        (*s).to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "non-string panic payload".to_string()
    }
}

/// Everything Phase 1/2 need before the per-token generation loop starts:
/// prompt built, tokenized, context-length checked, sampler ready, and the
/// prompt itself already prefilled into `context`. Shared by `run_complete`
/// and `run_stream` so the two can't drift on tokenization/prefill/sampler
/// setup - only the per-token handling and final-response shape differ.
struct PreparedGeneration {
    sampler: LlamaSampler,
    max_tokens: u32,
    prompt_token_count: u32,
}

fn prepare_generation(
    model: &LlamaModel,
    context: &mut LlamaContext<'_>,
    chat_template: &Option<LlamaChatTemplate>,
    sampling_defaults: &SamplingDefaults,
    default_seed: Option<u32>,
    request: &LLMRequest,
) -> Result<PreparedGeneration> {
    let prompt = build_prompt(model, chat_template, request)?;

    let prompt_tokens = model
        .str_to_token(&prompt, AddBos::Always)
        .map_err(|e| ProviderError::InvalidRequest(format!("failed to tokenize prompt: {e}")))?;

    let n_ctx = context.n_ctx();
    if prompt_tokens.len() as u32 >= n_ctx {
        return Err(ProviderError::ContextLengthExceeded(
            prompt_tokens.len() as u32
        ));
    }

    let max_tokens = request
        .max_tokens
        .unwrap_or(DEFAULT_MAX_TOKENS)
        .min(n_ctx.saturating_sub(prompt_tokens.len() as u32));

    let sampler = build_sampler(sampling_defaults, request, default_seed);

    // Prefill: decode the whole prompt in one batch. `add_sequence` sets
    // logits=true only on the last token, which is all prefill needs -
    // generation reads from that position via `sample(ctx, -1)`.
    let mut batch = LlamaBatch::new(prompt_tokens.len().max(1), 1);
    batch
        .add_sequence(&prompt_tokens, 0, false)
        .map_err(|e| ProviderError::InvalidRequest(format!("failed to build prompt batch: {e}")))?;
    context
        .decode(&mut batch)
        .map_err(|e| ProviderError::Internal(format!("prefill decode failed: {e}")))?;

    Ok(PreparedGeneration {
        sampler,
        max_tokens,
        prompt_token_count: prompt_tokens.len() as u32,
    })
}

/// Decode exactly one new token into `context` at `pos` for sequence 0,
/// mirroring the single-token extension step both `run_complete` and
/// `run_stream` need after each sampled token.
fn decode_one_more(
    context: &mut LlamaContext<'_>,
    batch: &mut LlamaBatch<'_>,
    token: LlamaToken,
    pos: i32,
) -> Result<()> {
    batch.clear();
    batch
        .add(token, pos, &[0], true)
        .map_err(|e| ProviderError::Internal(format!("failed to extend batch: {e}")))?;
    context
        .decode(batch)
        .map_err(|e| ProviderError::Internal(format!("decode failed: {e}")))
}

/// Build the prompt for `request`, tokenize it, run prefill + generation,
/// and translate the result into an `LLMResponse`. Runs entirely on the
/// worker thread - `context` is `&mut` because decoding advances its
/// internal KV-cache/state.
fn run_complete(
    model: &LlamaModel,
    context: &mut LlamaContext<'_>,
    chat_template: &Option<LlamaChatTemplate>,
    sampling_defaults: &SamplingDefaults,
    default_seed: Option<u32>,
    request: LLMRequest,
) -> Result<LLMResponse> {
    let start = std::time::Instant::now();

    let PreparedGeneration {
        mut sampler,
        max_tokens,
        prompt_token_count,
    } = prepare_generation(
        model,
        context,
        chat_template,
        sampling_defaults,
        default_seed,
        &request,
    )?;

    let stop_sequences = request.stop.clone().unwrap_or_default();
    let mut generated_bytes: Vec<u8> = Vec::new();
    let mut generated_count: u32 = 0;
    let mut stop_reason = StopReason::EndTurn;
    let mut pos = prompt_token_count as i32;
    let mut batch = LlamaBatch::new(1, 1);

    for _ in 0..max_tokens {
        let token = sampler.sample(context, -1);

        if model.is_eog_token(token) {
            stop_reason = StopReason::EndTurn;
            break;
        }

        if let Ok(piece) = token_to_piece_bytes(model, token) {
            generated_bytes.extend_from_slice(&piece);
        }
        generated_count += 1;

        if !stop_sequences.is_empty() {
            let text_so_far = String::from_utf8_lossy(&generated_bytes);
            if stop_sequences.iter().any(|s| text_so_far.ends_with(s.as_str())) {
                stop_reason = StopReason::StopSequence;
                break;
            }
        }

        if generated_count >= max_tokens {
            stop_reason = StopReason::MaxTokens;
            break;
        }

        decode_one_more(context, &mut batch, token, pos)?;
        pos += 1;
    }

    let text = String::from_utf8_lossy(&generated_bytes).into_owned();

    // `llama.cpp` has no native tool-calling field to check first (unlike
    // Ollama) - recovery from printed JSON is the *only* way a call is ever
    // recognized here. See tool_call_recovery.rs and §4.7.
    let offered_tools = request.tools.clone().unwrap_or_default();
    let recovered = if offered_tools.is_empty() {
        None
    } else {
        tool_call_from_content(&text, &offered_tools)
    };

    let mut content = Vec::new();
    if recovered.is_none() && !text.is_empty() {
        content.push(ContentBlock::Text { text });
    }
    if let Some((name, input)) = &recovered {
        content.push(ContentBlock::ToolUse {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.clone(),
            input: input.clone(),
        });
        stop_reason = StopReason::ToolUse;
    }

    let timings = context.timings();
    let total_ms = start.elapsed().as_millis() as u64;

    Ok(LLMResponse {
        id: format!("llama-cpp-{}", uuid::Uuid::new_v4()),
        model: request.model,
        content,
        stop_reason: Some(stop_reason),
        usage: TokenUsage {
            input_tokens: prompt_token_count,
            output_tokens: generated_count,
        },
        cache_metrics: None,
        perf_metrics: Some(PerfMetrics {
            load_duration_ms: Some(timings.t_load_ms().max(0.0) as u64),
            prompt_eval_duration_ms: Some(timings.t_p_eval_ms().max(0.0) as u64),
            eval_duration_ms: Some(timings.t_eval_ms().max(0.0) as u64),
            total_duration_ms: Some(total_ms),
            model_was_loaded: Some(timings.t_load_ms() <= 0.0),
        }),
    })
}

/// Streaming counterpart of `run_complete`: pushes `StreamEvent`s through
/// `events_tx` as each token is generated instead of collecting a final
/// `LLMResponse`. Returns once the stream is finished (EOS/max
/// tokens/stop-sequence/consumer gone) - the caller (`worker_loop`) doesn't
/// wait for this to know the request was *accepted* (that already happened
/// when `stream()` sent the job), only to know the worker is free again.
#[allow(clippy::too_many_arguments)]
fn run_stream(
    model: &LlamaModel,
    context: &mut LlamaContext<'_>,
    chat_template: &Option<LlamaChatTemplate>,
    sampling_defaults: &SamplingDefaults,
    default_seed: Option<u32>,
    request: LLMRequest,
    events_tx: &mpsc::UnboundedSender<Result<StreamEvent>>,
) {
    let start = std::time::Instant::now();
    let message_id = format!("llama-cpp-{}", uuid::Uuid::new_v4());
    let model_name = request.model.clone();

    let prepared = prepare_generation(
        model,
        context,
        chat_template,
        sampling_defaults,
        default_seed,
        &request,
    );
    let PreparedGeneration {
        mut sampler,
        max_tokens,
        prompt_token_count,
    } = match prepared {
        Ok(p) => p,
        Err(e) => {
            // Setup failed before any event was sent - nothing to unwind,
            // just report the error as the only item on the stream.
            let _ = events_tx.send(Err(e));
            return;
        }
    };

    if events_tx
        .send(Ok(StreamEvent::MessageStart {
            message: StreamMessage {
                id: message_id.clone(),
                model: model_name.clone(),
                role: Role::Assistant,
                usage: TokenUsage {
                    input_tokens: prompt_token_count,
                    output_tokens: 0,
                },
            },
        }))
        .is_err()
    {
        return; // consumer already gone
    }
    // The text ContentBlockStart is sent lazily, on the first delta that
    // actually gets flushed (below) - not here. When tools are offered and
    // the whole response turns out to be a recovered tool call, no text
    // block is ever shown to the user at all, matching OllamaProvider's own
    // streaming behavior for the same case.

    let offered_tools = request.tools.clone().unwrap_or_default();
    let stop_sequences = request.stop.clone().unwrap_or_default();
    let mut pending_utf8: Vec<u8> = Vec::new();
    let mut full_text = String::new();
    // Decoded text not yet flushed as a delta - withheld while it still
    // might turn out to be a tool call printed as JSON (§4.7). Reset to
    // empty every time it's actually flushed.
    let mut pending_flush = String::new();
    let mut text_block_started = false;
    let mut generated_count: u32 = 0;
    let mut stop_reason = StopReason::EndTurn;
    let mut pos = prompt_token_count as i32;
    let mut batch = LlamaBatch::new(1, 1);

    'generate: for _ in 0..max_tokens {
        let token = sampler.sample(context, -1);

        if model.is_eog_token(token) {
            stop_reason = StopReason::EndTurn;
            break;
        }

        if let Ok(piece) = token_to_piece_bytes(model, token) {
            pending_utf8.extend_from_slice(&piece);
        }
        generated_count += 1;

        if let Some(chunk) = drain_valid_utf8(&mut pending_utf8) {
            full_text.push_str(&chunk);
            pending_flush.push_str(&chunk);

            let might_be_tool_call =
                !offered_tools.is_empty() && maybe_tool_call_json(&pending_flush);
            if !might_be_tool_call {
                if !text_block_started {
                    text_block_started = true;
                    if events_tx
                        .send(Ok(StreamEvent::ContentBlockStart {
                            index: 0,
                            content_block: ContentBlock::Text {
                                text: String::new(),
                            },
                        }))
                        .is_err()
                    {
                        return;
                    }
                }
                if events_tx
                    .send(Ok(StreamEvent::ContentBlockDelta {
                        index: 0,
                        delta: ContentDelta::TextDelta {
                            text: std::mem::take(&mut pending_flush),
                        },
                    }))
                    .is_err()
                {
                    // Consumer dropped the stream (cancelled) - stop
                    // generating rather than burning CPU on tokens nobody
                    // will see, and return to the worker's job loop instead
                    // of hanging.
                    return;
                }
            }
        }

        if !stop_sequences.is_empty()
            && stop_sequences.iter().any(|s| full_text.ends_with(s.as_str()))
        {
            stop_reason = StopReason::StopSequence;
            break 'generate;
        }

        if generated_count >= max_tokens {
            stop_reason = StopReason::MaxTokens;
            break;
        }

        if let Err(e) = decode_one_more(context, &mut batch, token, pos) {
            let _ = events_tx.send(Err(e));
            return;
        }
        pos += 1;
    }

    // Flush any trailing bytes that never completed a UTF-8 sequence
    // (generation ended mid-token) into the withheld buffer - lossy, but
    // this is the same edge case `run_complete`'s final `from_utf8_lossy`
    // already accepts.
    if !pending_utf8.is_empty() {
        let chunk = String::from_utf8_lossy(&pending_utf8).into_owned();
        full_text.push_str(&chunk);
        pending_flush.push_str(&chunk);
    }

    // Whatever is still withheld at the end is the only candidate for a
    // recovered tool call - `llama.cpp` has no native tool-calling field,
    // unlike Ollama, so this printed-JSON recovery is the only mechanism.
    let recovered = if offered_tools.is_empty() {
        None
    } else {
        tool_call_from_content(&pending_flush, &offered_tools)
    };

    if recovered.is_none() && !pending_flush.is_empty() {
        // Turned out not to be a tool call after all - flush it now rather
        // than silently dropping it.
        if !text_block_started {
            text_block_started = true;
            let _ = events_tx.send(Ok(StreamEvent::ContentBlockStart {
                index: 0,
                content_block: ContentBlock::Text {
                    text: String::new(),
                },
            }));
        }
        let _ = events_tx.send(Ok(StreamEvent::ContentBlockDelta {
            index: 0,
            delta: ContentDelta::TextDelta {
                text: pending_flush,
            },
        }));
    }

    if text_block_started {
        let _ = events_tx.send(Ok(StreamEvent::ContentBlockStop { index: 0 }));
    }

    if let Some((name, input)) = &recovered {
        // Comes after any text block, offset by 1 if one was started -
        // matches OllamaProvider::stream()'s own indexing for tool-use
        // blocks that follow streamed text.
        let tool_index = usize::from(text_block_started);
        let _ = events_tx.send(Ok(StreamEvent::ContentBlockStart {
            index: tool_index,
            content_block: ContentBlock::ToolUse {
                id: uuid::Uuid::new_v4().to_string(),
                name: name.clone(),
                input: input.clone(),
            },
        }));
        let _ = events_tx.send(Ok(StreamEvent::ContentBlockStop { index: tool_index }));
        stop_reason = StopReason::ToolUse;
    }

    let timings = context.timings();
    let total_ms = start.elapsed().as_millis() as u64;
    let _ = events_tx.send(Ok(StreamEvent::MessageDelta {
        delta: MessageDelta {
            stop_reason: Some(stop_reason),
            stop_sequence: None,
        },
        usage: TokenUsage {
            input_tokens: prompt_token_count,
            output_tokens: generated_count,
        },
        perf_metrics: Some(PerfMetrics {
            load_duration_ms: Some(timings.t_load_ms().max(0.0) as u64),
            prompt_eval_duration_ms: Some(timings.t_p_eval_ms().max(0.0) as u64),
            eval_duration_ms: Some(timings.t_eval_ms().max(0.0) as u64),
            total_duration_ms: Some(total_ms),
            model_was_loaded: Some(timings.t_load_ms() <= 0.0),
        }),
    }));
    let _ = events_tx.send(Ok(StreamEvent::MessageStop));
}

/// Incrementally decode as much valid UTF-8 as possible from `buffer`,
/// leaving any trailing incomplete multi-byte sequence for the next call to
/// complete (a token's raw bytes are not guaranteed to end on a UTF-8
/// character boundary). Returns `None` when nothing new is decodable yet.
fn drain_valid_utf8(buffer: &mut Vec<u8>) -> Option<String> {
    if buffer.is_empty() {
        return None;
    }
    let valid_up_to = match std::str::from_utf8(buffer) {
        Ok(_) => buffer.len(),
        Err(e) => e.valid_up_to(),
    };
    if valid_up_to == 0 {
        return None;
    }
    let decoded = String::from_utf8_lossy(&buffer[..valid_up_to]).into_owned();
    buffer.drain(..valid_up_to);
    Some(decoded)
}

/// Convert a token to its raw UTF-8 bytes, retrying with a larger buffer if
/// `llama.cpp` reports the initial one was too small (mirrors the retry
/// pattern `LlamaModel::token_to_piece` itself uses internally).
fn token_to_piece_bytes(
    model: &LlamaModel,
    token: LlamaToken,
) -> std::result::Result<Vec<u8>, String> {
    match model.token_to_piece_bytes(token, 8, false, None) {
        Ok(bytes) => Ok(bytes),
        Err(llama_cpp_2::TokenToStringError::InsufficientBufferSpace(needed)) => model
            .token_to_piece_bytes(token, needed.unsigned_abs() as usize, false, None)
            .map_err(|e| e.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

/// Build the sampler chain for one request: request-level overrides win,
/// falling back to the provider's configured defaults. `temperature = 0.0`
/// selects greedy (deterministic) decoding, matching common llama.cpp
/// front-end convention.
fn build_sampler(
    defaults: &SamplingDefaults,
    request: &LLMRequest,
    default_seed: Option<u32>,
) -> LlamaSampler {
    let temperature = request.temperature.unwrap_or(defaults.temperature);
    let top_p = request.top_p.unwrap_or(defaults.top_p);
    // A seed that doesn't fit in u32 is dropped rather than silently
    // truncated (truncation would reproduce a *different* sequence than the
    // one the caller actually asked for, which is worse than falling back
    // to the provider default/random the same way an absent seed does) -
    // mirrors `OllamaProvider::to_ollama_request`'s `i32::try_from` handling
    // of the same generic `LLMRequest.seed` field.
    let seed = request
        .seed
        .and_then(|s| u32::try_from(s).ok())
        .or(default_seed)
        .unwrap_or_else(rand::random);
    // 0.0 = disabled, matching both llama.cpp's own convention for these
    // penalties and `LLMRequest`'s documented OpenAI-compatible semantics.
    let frequency_penalty = request.frequency_penalty.unwrap_or(0.0);
    let presence_penalty = request.presence_penalty.unwrap_or(0.0);

    if temperature <= 0.0 {
        return LlamaSampler::chain([LlamaSampler::greedy()], false);
    }

    LlamaSampler::chain(
        [
            LlamaSampler::penalties(
                64,
                defaults.repeat_penalty,
                frequency_penalty,
                presence_penalty,
            ),
            LlamaSampler::top_k(defaults.top_k),
            LlamaSampler::top_p(top_p, 1),
            LlamaSampler::temp(temperature),
            LlamaSampler::dist(seed),
        ],
        false,
    )
}

/// Render the offered tools as a system-prompt instruction block.
/// `llama.cpp` has no native tool-calling API (unlike Ollama's `tool_calls`
/// field) - the model must be told, in the prompt itself, what tools exist
/// and the exact JSON shape to answer with when it wants to call one. The
/// response is then checked with `tool_call_recovery::tool_call_from_content`
/// after generation - this block is what makes that check likely to find
/// something, not what enforces it.
fn tool_instructions_block(tools: &[Tool]) -> String {
    let mut block = String::from(
        "You have access to the following tools. To call one, respond with \
         ONLY a single JSON object of the form {\"name\": \"<tool_name>\", \
         \"arguments\": {...}} and nothing else - no explanation, no code \
         fence, no extra text. If you don't need a tool, just answer \
         normally.\n\nAvailable tools:\n",
    );
    for tool in tools {
        block.push_str(&format!(
            "- {}: {}\n  Parameters (JSON Schema): {}\n",
            tool.name, tool.description, tool.input_schema
        ));
    }
    block
}

/// Combine a request's `system` prompt with the tool-instructions block
/// (§4.7), if any tools were offered. Pure and offline-testable on its own,
/// separate from `build_prompt`'s `LlamaModel`-dependent chat-template
/// logic.
fn merged_system_prompt(system: Option<&str>, tools: Option<&[Tool]>) -> Option<String> {
    let tools_block = tools.filter(|t| !t.is_empty()).map(|t| tool_instructions_block(t));
    match (system, tools_block) {
        (Some(system), Some(tools)) => Some(format!("{system}\n\n{tools}")),
        (Some(system), None) => Some(system.to_string()),
        (None, Some(tools)) => Some(tools),
        (None, None) => None,
    }
}

/// Build the prompt text for `request`: applies the resolved chat template
/// if one is available, otherwise falls back to a minimal manual
/// role-prefixed transcript (ChatML-style) so a model without a usable
/// embedded template is still usable, just less reliably formatted.
fn build_prompt(
    model: &LlamaModel,
    chat_template: &Option<LlamaChatTemplate>,
    request: &LLMRequest,
) -> Result<String> {
    let mut chat_messages = Vec::new();
    if let Some(system) = merged_system_prompt(request.system.as_deref(), request.tools.as_deref())
    {
        chat_messages.push(("system", system));
    }
    for msg in &request.messages {
        let role = match msg.role {
            Role::System => "system",
            Role::User => "user",
            Role::Assistant => "assistant",
        };
        let text: String = msg
            .content
            .iter()
            .filter_map(|block| match block {
                ContentBlock::Text { text } => Some(text.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("\n");
        if !text.is_empty() {
            chat_messages.push((role, text));
        }
    }

    if let Some(tmpl) = chat_template {
        let llama_messages: std::result::Result<Vec<LlamaChatMessage>, _> = chat_messages
            .iter()
            .map(|(role, content)| LlamaChatMessage::new(role.to_string(), content.clone()))
            .collect();
        if let Ok(llama_messages) = llama_messages {
            if let Ok(prompt) = model.apply_chat_template(tmpl, &llama_messages, true) {
                return Ok(prompt);
            }
        }
        // Fall through to the manual fallback below on any conversion/apply
        // failure - malformed template output must not abort the request.
    }

    let mut prompt = String::new();
    for (role, content) in &chat_messages {
        prompt.push_str(&format!("<|{role}|>\n{content}\n"));
    }
    prompt.push_str("<|assistant|>\n");
    Ok(prompt)
}

#[async_trait]
impl Provider for LlamaCppProvider {
    async fn complete(&self, request: LLMRequest) -> Result<LLMResponse> {
        let (respond_to, response_rx) = oneshot::channel();
        self.job_tx
            .send(InferenceJob::Complete {
                request,
                respond_to,
            })
            .map_err(|_| {
                ProviderError::Internal("llama.cpp worker thread is no longer running".to_string())
            })?;

        response_rx.await.map_err(|_| {
            ProviderError::Internal(
                "llama.cpp worker thread dropped the request without responding".to_string(),
            )
        })?
    }

    async fn stream(&self, request: LLMRequest) -> Result<ProviderStream> {
        let (events_tx, events_rx) = mpsc::unbounded_channel::<Result<StreamEvent>>();
        self.job_tx
            .send(InferenceJob::Stream { request, events_tx })
            .map_err(|_| {
                ProviderError::Internal("llama.cpp worker thread is no longer running".to_string())
            })?;

        // Returned immediately, before generation starts - each StreamEvent
        // is pushed by the worker thread as it's produced (see `run_stream`),
        // not buffered and replayed after the fact.
        Ok(Box::pin(
            tokio_stream::wrappers::UnboundedReceiverStream::new(events_rx),
        ))
    }

    fn supports_streaming(&self) -> bool {
        true
    }

    fn supports_tools(&self) -> bool {
        // Recovery-based, like OllamaProvider's fallback path: llama.cpp has
        // no native tool-calling field, so a call is only ever recognized by
        // parsing printed JSON (tool_call_recovery.rs, §4.7). No
        // grammar-constrained guarantee yet (Phase 4b, llguidance).
        true
    }

    fn supports_vision(&self) -> bool {
        // Deferred (llama-cpp-2-integration-plan.md §4.9) - not a claim
        // that mtmd/multimodal is unsupportable, just not built yet.
        false
    }

    fn name(&self) -> &str {
        "llama-cpp"
    }

    fn default_model(&self) -> &str {
        &self.display_name
    }

    fn supported_models(&self) -> Vec<String> {
        // One provider instance is bound to exactly one loaded GGUF file
        // (llama-cpp-2-integration-plan.md §10.2) - there is no registry of
        // other models to list.
        vec![self.display_name.clone()]
    }

    fn validate_model(&self, _model: &str) -> bool {
        // Every request is served by the one loaded model regardless of
        // what `model` string it names - see §4.10 of the integration plan
        // for why a request naming a different model doesn't (and can't)
        // change what actually answers.
        true
    }

    fn context_window(&self, _model: &str) -> Option<u32> {
        Some(self.n_ctx)
    }

    fn calculate_cost(&self, _model: &str, _input_tokens: u32, _output_tokens: u32) -> f64 {
        // Local inference: no per-token API cost.
        0.0
    }
}

impl std::fmt::Debug for LlamaCppProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LlamaCppProvider")
            .field("model_path", &self.model_path)
            .field("display_name", &self.display_name)
            .field("n_ctx", &self.n_ctx)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_reports_model_not_found_for_a_missing_path() {
        let config = crate::config::LlamaCppProviderConfig {
            model_path: PathBuf::from("/nonexistent/path/to/model.gguf"),
            ..Default::default()
        };

        let err = LlamaCppProvider::new(&config).expect_err("missing file must error");
        assert!(matches!(err, ProviderError::ModelNotFound(_)));
    }

    #[test]
    fn display_name_defaults_to_the_file_stem() {
        // Exercises the same file-stem derivation `new()` uses, without
        // requiring a real model load (covered instead by the manual test
        // plan - llama-cpp-2-integration-plan.md §12.5).
        let path = PathBuf::from("/models/qwen2.5-coder-7b-instruct-q4_k_m.gguf");
        let derived = path
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_default();
        assert_eq!(derived, "qwen2.5-coder-7b-instruct-q4_k_m");
    }

    #[test]
    fn sampling_defaults_match_documented_values() {
        let d = SamplingDefaults::default();
        assert_eq!(d.temperature, 0.8);
        assert_eq!(d.top_p, 0.95);
        assert_eq!(d.top_k, 40);
        assert_eq!(d.repeat_penalty, 1.1);
    }

    // ── drain_valid_utf8 (Phase 2 streaming) ──────────────────────────────

    #[test]
    fn drain_valid_utf8_full_ascii_chunk() {
        let mut buf = b"hello".to_vec();
        assert_eq!(drain_valid_utf8(&mut buf), Some("hello".to_string()));
        assert!(buf.is_empty());
    }

    #[test]
    fn drain_valid_utf8_empty_buffer_returns_none() {
        let mut buf = Vec::new();
        assert_eq!(drain_valid_utf8(&mut buf), None);
    }

    #[test]
    fn drain_valid_utf8_holds_back_an_incomplete_multibyte_sequence() {
        // "é" is 2 bytes (0xC3 0xA9) in UTF-8 - split across two "tokens"
        // the way llama.cpp's per-token pieces can.
        let full = "café".as_bytes().to_vec();
        let (first, second) = full.split_at(full.len() - 1); // split mid "é"

        let mut buf = first.to_vec();
        // "caf" is complete; the lone lead byte of "é" must be held back.
        assert_eq!(drain_valid_utf8(&mut buf), Some("caf".to_string()));
        assert_eq!(buf.len(), 1, "the incomplete lead byte must be retained");

        buf.extend_from_slice(second);
        assert_eq!(drain_valid_utf8(&mut buf), Some("é".to_string()));
        assert!(buf.is_empty());
    }

    #[test]
    fn drain_valid_utf8_multiple_tokens_reassemble_correctly() {
        // Simulates token-by-token delivery of a multi-byte-heavy string.
        let pieces: Vec<Vec<u8>> = "日本語"
            .as_bytes()
            .chunks(2) // deliberately misaligned with 3-byte CJK boundaries
            .map(|c| c.to_vec())
            .collect();

        let mut buf = Vec::new();
        let mut reassembled = String::new();
        for piece in pieces {
            buf.extend_from_slice(&piece);
            if let Some(chunk) = drain_valid_utf8(&mut buf) {
                reassembled.push_str(&chunk);
            }
        }
        assert!(buf.is_empty(), "no bytes should be stranded at the end");
        assert_eq!(reassembled, "日本語");
    }

    #[test]
    fn drain_valid_utf8_never_panics_on_arbitrary_bytes() {
        // Not a claim of correctness for garbage input - just that it can't
        // panic on the worker thread mid-stream.
        for byte in 0u8..=255 {
            let mut buf = vec![byte, byte, byte];
            let _ = drain_valid_utf8(&mut buf);
        }
    }

    // ── tool-prompt construction (Phase 4) ────────────────────────────────

    fn bash_tool() -> Tool {
        Tool {
            name: "bash".to_string(),
            description: "Run a shell command".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {"command": {"type": "string"}},
                "required": ["command"],
            }),
        }
    }

    #[test]
    fn merged_system_prompt_with_neither_is_none() {
        assert_eq!(merged_system_prompt(None, None), None);
    }

    #[test]
    fn merged_system_prompt_system_only_is_unchanged() {
        assert_eq!(
            merged_system_prompt(Some("be terse"), None),
            Some("be terse".to_string())
        );
    }

    #[test]
    fn merged_system_prompt_empty_tools_list_behaves_like_none() {
        // An empty `tools: Some(vec![])` (as opposed to `None`) must not
        // inject an empty/useless instructions block.
        assert_eq!(
            merged_system_prompt(Some("be terse"), Some(&[])),
            Some("be terse".to_string())
        );
        assert_eq!(merged_system_prompt(None, Some(&[])), None);
    }

    #[test]
    fn merged_system_prompt_tools_only_still_produces_instructions() {
        let tools = [bash_tool()];
        let merged = merged_system_prompt(None, Some(&tools)).expect("tools alone must inject");
        assert!(merged.contains("bash"));
        assert!(merged.contains("Run a shell command"));
    }

    #[test]
    fn merged_system_prompt_combines_system_and_tools_with_system_first() {
        let tools = [bash_tool()];
        let merged = merged_system_prompt(Some("be terse"), Some(&tools)).expect("both present");
        let system_pos = merged.find("be terse").expect("system text present");
        let tools_pos = merged.find("bash").expect("tool name present");
        assert!(
            system_pos < tools_pos,
            "system prompt must come before the tool instructions"
        );
    }

    #[test]
    fn tool_instructions_block_names_every_offered_tool() {
        let tools = [
            bash_tool(),
            Tool {
                name: "read_file".to_string(),
                description: "Read a file".to_string(),
                input_schema: serde_json::json!({"type": "object"}),
            },
        ];
        let block = tool_instructions_block(&tools);
        assert!(block.contains("bash"));
        assert!(block.contains("read_file"));
        // The instruction to answer with bare JSON must be present, or the
        // model has no idea what format to use.
        assert!(block.contains("\"name\""));
        assert!(block.contains("\"arguments\""));
    }
}
