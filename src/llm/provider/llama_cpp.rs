//! In-process `llama.cpp` provider (loads a local `.gguf` file directly).
//!
//! Unlike every other provider in this module, this one is not an HTTP
//! client - it loads model weights and runs inference inside this process,
//! via the `llama-cpp-2` crate (FFI bindings to `llama.cpp`). See
//! `llama-cpp-2-integration-plan.md` for the full design; this file
//! implements Phase 1 (non-streaming `complete()` only, CPU-only, no tool
//! calling yet - `stream()` (Phase 2), sampling/config knobs beyond the
//! basics (Phase 3), and tool-call recovery (Phase 4) land in later phases).
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
struct InferenceJob {
    request: LLMRequest,
    respond_to: oneshot::Sender<Result<LLMResponse>>,
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
        let (ready_tx, ready_rx) = std::sync::mpsc::channel::<std::result::Result<(), String>>();

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

        std::thread::spawn(move || {
            worker_loop(WorkerInit {
                model_path,
                n_ctx,
                n_gpu_layers,
                n_threads,
                chat_template_override,
                sampling_defaults,
                seed,
                job_rx,
                ready_tx,
            });
        });

        match ready_rx.recv() {
            Ok(Ok(())) => Ok(Self {
                job_tx,
                model_path: config.model_path.clone(),
                display_name,
                n_ctx: config.n_ctx,
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
    job_rx: mpsc::UnboundedReceiver<InferenceJob>,
    ready_tx: std::sync::mpsc::Sender<std::result::Result<(), String>>,
}

/// The worker thread body: loads the model once, then serially drains
/// `InferenceJob`s until the sender is dropped. Every `llama.cpp` FFI call
/// in this provider happens inside this function or the functions it calls
/// - never on the async caller's thread. See the module doc for why.
fn worker_loop(init: WorkerInit) {
    let WorkerInit {
        model_path,
        n_ctx,
        n_gpu_layers,
        n_threads,
        chat_template_override,
        sampling_defaults,
        seed,
        mut job_rx,
        ready_tx,
    } = init;

    let backend = match LlamaBackend::init() {
        Ok(b) => b,
        Err(e) => {
            let _ = ready_tx.send(Err(format!("failed to init llama.cpp backend: {e:?}")));
            return;
        }
    };

    let model_params = LlamaModelParams::default().with_n_gpu_layers(n_gpu_layers);
    let model = match LlamaModel::load_from_file(&backend, &model_path, &model_params) {
        Ok(m) => m,
        Err(e) => {
            let _ = ready_tx.send(Err(format!("failed to load GGUF file: {e:?}")));
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
            let _ = ready_tx.send(Err(format!("failed to create llama.cpp context: {e:?}")));
            return;
        }
    };

    // Resolve the chat template once: an explicit config override always
    // wins; otherwise use the model's own embedded GGUF template if
    // present. A model with neither gets a minimal manual fallback
    // assembled per-request in `build_prompt` - not stored here since it
    // isn't a `LlamaChatTemplate`.
    let chat_template: Option<LlamaChatTemplate> = chat_template_override
        .as_deref()
        .and_then(|t| LlamaChatTemplate::new(t).ok())
        .or_else(|| model.chat_template(None).ok());

    // Loading succeeded - tell `new()` it can return `Ok`.
    if ready_tx.send(Ok(())).is_err() {
        // The caller gave up waiting (e.g. it timed out or the process is
        // shutting down) - nothing to serve, exit quietly.
        return;
    }

    while let Some(job) = job_rx.blocking_recv() {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            run_complete(
                &model,
                &mut context,
                &chat_template,
                &sampling_defaults,
                seed,
                job.request,
            )
        }));
        let result = result.unwrap_or_else(|payload| {
            let msg = panic_message(&payload);
            tracing::error!("llama.cpp worker panicked during a request: {msg}");
            Err(ProviderError::Internal(format!(
                "llama.cpp inference panicked: {msg}"
            )))
        });
        let _ = job.respond_to.send(result);
    }
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

    let prompt = build_prompt(model, chat_template, &request)?;

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

    let mut sampler = build_sampler(sampling_defaults, &request, default_seed);

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

    let stop_sequences = request.stop.clone().unwrap_or_default();
    let mut generated_bytes: Vec<u8> = Vec::new();
    let mut generated_count: u32 = 0;
    let mut stop_reason = StopReason::EndTurn;
    let mut pos = prompt_tokens.len() as i32;

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

        batch.clear();
        batch
            .add(token, pos, &[0], true)
            .map_err(|e| ProviderError::Internal(format!("failed to extend batch: {e}")))?;
        context
            .decode(&mut batch)
            .map_err(|e| ProviderError::Internal(format!("decode failed: {e}")))?;
        pos += 1;
    }

    let text = String::from_utf8_lossy(&generated_bytes).into_owned();
    let timings = context.timings();
    let total_ms = start.elapsed().as_millis() as u64;

    Ok(LLMResponse {
        id: format!("llama-cpp-{}", uuid::Uuid::new_v4()),
        model: request.model,
        content: vec![ContentBlock::Text { text }],
        stop_reason: Some(stop_reason),
        usage: TokenUsage {
            input_tokens: prompt_tokens.len() as u32,
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
    let seed = request
        .seed
        .map(|s| s as u32)
        .or(default_seed)
        .unwrap_or_else(rand::random);

    if temperature <= 0.0 {
        return LlamaSampler::chain([LlamaSampler::greedy()], false);
    }

    LlamaSampler::chain(
        [
            LlamaSampler::penalties(64, defaults.repeat_penalty, 0.0, 0.0),
            LlamaSampler::top_k(defaults.top_k),
            LlamaSampler::top_p(top_p, 1),
            LlamaSampler::temp(temperature),
            LlamaSampler::dist(seed),
        ],
        false,
    )
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
    if let Some(system) = &request.system {
        chat_messages.push(("system", system.clone()));
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
            .send(InferenceJob {
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

    async fn stream(&self, _request: LLMRequest) -> Result<ProviderStream> {
        // Phase 2 (llama-cpp-2-integration-plan.md §13): token-by-token
        // streaming through the worker thread. Not yet implemented.
        Err(ProviderError::StreamingNotSupported)
    }

    fn supports_streaming(&self) -> bool {
        false
    }

    fn supports_tools(&self) -> bool {
        // Phase 4 (tool-call recovery, mirroring OllamaProvider's approach)
        // is not implemented yet.
        false
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
}
