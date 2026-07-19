//! Ollama Provider Implementation (native API, via `ollama-rs`)
//!
//! Implements the `Provider` trait against Ollama's native `/api/chat`
//! protocol (as opposed to `OpenAIProvider::local()`, which talks to
//! Ollama's OpenAI-compatible `/v1/chat/completions` shim). Both providers
//! can be used side by side; this one additionally exposes:
//!
//! - `keep_alive` / `num_ctx` control
//! - Runtime performance metrics (`PerfMetrics`): model load time, prefill
//!   and generation durations, derived tokens/second — none of these are
//!   present in Ollama's OpenAI-compatible response format.
//!
//! ## Note on error handling
//!
//! `ollama-rs` is built against `reqwest` 0.12, while the rest of this crate
//! (and `super::error::ProviderError::HttpError`) uses `reqwest` 0.11 — the
//! two major versions are distinct, incompatible Rust types and coexist in
//! the dependency tree without being shared. Because of this, network-level
//! errors from `ollama-rs` cannot be wrapped in `ProviderError::HttpError`
//! and are mapped to `ProviderError::ApiError` instead (see
//! `map_ollama_error`). As a consequence this provider does not go through
//! `retry::retry_with_backoff` in this initial version: `ApiError { status:
//! 0, .. }` is not classified as retryable, so a manual retry wrapper would
//! be a no-op. Revisit if/when `ollama-rs` migrates to `reqwest` 0.11 or
//! `ProviderError` grows a transport-agnostic "network error" variant.

use super::error::{ProviderError, Result};
use super::r#trait::{Provider, ProviderStream};
use super::types::*;
use async_trait::async_trait;
use ollama_rs::{
    error::OllamaError,
    generation::chat::{
        request::ChatMessageRequest, ChatMessage, ChatMessageFinalResponseData,
        ChatMessageResponse, MessageRole,
    },
    generation::images::Image,
    generation::parameters::{FormatType, JsonStructure, KeepAlive, ThinkType, TimeUnit},
    generation::tools::{ToolCall, ToolCallFunction, ToolFunctionInfo, ToolInfo, ToolType},
    models::ModelOptions,
    Ollama,
};

/// Default local Ollama host, matching the `OLLAMA_HOST` CLI convention.
const DEFAULT_OLLAMA_HOST: &str = "http://127.0.0.1:11434";

/// Context window assumed - and, critically, actually requested via `num_ctx`
/// - when the user hasn't configured `providers.ollama.num_ctx`.
///
/// Ollama silently truncates the oldest turns server-side once its own
/// running context fills up, which is commonly *smaller* than what
/// `context_window()` used to unconditionally report (8192) for compaction
/// bookkeeping: `to_ollama_request()` only ever sent `num_ctx` when this
/// field was configured, so an unconfigured install ran at whatever
/// Ollama/the model's Modelfile defaults to - not the 8192 Crustly's own
/// compaction threshold assumed there was room for. That mismatch meant
/// context could be silently dropped by Ollama before Crustly's own
/// compaction ever got a chance to summarize it. Defaulting `num_ctx` itself
/// to this same constant (see `OllamaProvider::new`) guarantees what's
/// requested and what's assumed can never drift apart.
const DEFAULT_NUM_CTX: u64 = 8_192;

/// Per-model overrides for Ollama sampling/context. Any field left `None`
/// falls back to the provider-level default. Keyed by exact model name.
#[derive(Clone, Debug, Default)]
pub struct ModelOverrides {
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub top_k: Option<u32>,
    pub num_ctx: Option<u64>,
    pub keep_alive: Option<KeepAlive>,
}

impl ModelOverrides {
    /// Build from raw config values, parsing the `keep_alive` string with the
    /// same rules as `with_keep_alive` (an invalid value is logged and dropped,
    /// so it falls back to the provider-level default rather than erroring).
    pub fn from_config(
        temperature: Option<f32>,
        top_p: Option<f32>,
        top_k: Option<u32>,
        num_ctx: Option<u32>,
        keep_alive: Option<&str>,
    ) -> Self {
        let keep_alive = keep_alive.and_then(|s| match parse_keep_alive(s) {
            Some(ka) => Some(ka),
            None => {
                tracing::warn!("Invalid per-model keep_alive value '{}', ignoring", s);
                None
            }
        });
        Self {
            temperature,
            top_p,
            top_k,
            num_ctx: num_ctx.map(|c| c as u64),
            keep_alive,
        }
    }
}

/// Ollama provider using the native `/api/chat` protocol.
#[derive(Clone)]
pub struct OllamaProvider {
    client: Ollama,
    custom_default_model: Option<String>,
    keep_alive: Option<KeepAlive>,
    num_ctx: Option<u64>,
    /// Sampling defaults applied when the request does not specify its own.
    ///
    /// Without these, Ollama falls back to its own generic defaults
    /// (temperature 0.8, top_p 0.9, top_k 40), which are not what a given model
    /// was tuned for - Ornith-1.0, for instance, documents 0.6 / 0.95 / 20.
    temperature: Option<f32>,
    top_p: Option<f32>,
    top_k: Option<u32>,
    /// Per-model overrides, keyed by exact model name. A model present here
    /// gets its own sampling/context; anything unset per-model falls back to
    /// the provider-level fields above. Different Ollama models want different
    /// tuning, so a single global set silently degrades all but one.
    per_model: std::collections::HashMap<String, ModelOverrides>,
}

impl OllamaProvider {
    /// Create a provider pointing at the default local Ollama instance
    /// (`http://127.0.0.1:11434`).
    pub fn default_local() -> Self {
        Self::new(DEFAULT_OLLAMA_HOST)
    }

    /// Create a provider pointing at a custom Ollama host, e.g.
    /// `http://localhost:11434` or a remote instance.
    ///
    /// Falls back to the default local host if `host` fails to parse as a
    /// URL, to avoid a hard panic on a malformed config value.
    pub fn new(host: impl Into<String>) -> Self {
        let host = host.into();
        let client = Ollama::try_new(host.clone()).unwrap_or_else(|e| {
            tracing::warn!(
                "Invalid Ollama host '{}' ({e}); falling back to {}",
                host,
                DEFAULT_OLLAMA_HOST
            );
            Ollama::default()
        });

        Self {
            client,
            custom_default_model: None,
            keep_alive: None,
            num_ctx: Some(DEFAULT_NUM_CTX),
            temperature: None,
            top_p: None,
            top_k: None,
            per_model: std::collections::HashMap::new(),
        }
    }

    /// Register per-model sampling/context overrides. Replaces any previously
    /// set map. See [`ModelOverrides`].
    pub fn with_per_model(
        mut self,
        per_model: std::collections::HashMap<String, ModelOverrides>,
    ) -> Self {
        self.per_model = per_model;
        self
    }

    /// Resolve the effective override for `model` field-by-field: a per-model
    /// value wins, otherwise the provider-level default. `num_ctx` keeps its
    /// existing behaviour of defaulting to `DEFAULT_NUM_CTX` when nothing is
    /// configured at either level (set in `new`).
    fn overrides_for(&self, model: &str) -> ModelOverrides {
        let m = self.per_model.get(model);
        ModelOverrides {
            temperature: m.and_then(|o| o.temperature).or(self.temperature),
            top_p: m.and_then(|o| o.top_p).or(self.top_p),
            top_k: m.and_then(|o| o.top_k).or(self.top_k),
            num_ctx: m.and_then(|o| o.num_ctx).or(self.num_ctx),
            keep_alive: m
                .and_then(|o| o.keep_alive.clone())
                .or(self.keep_alive.clone()),
        }
    }

    /// Set custom default model (e.g. `qwen2.5-coder:7b`).
    pub fn with_default_model(mut self, model: String) -> Self {
        self.custom_default_model = Some(model);
        self
    }

    /// Control how long the model stays loaded in memory after this
    /// request. Accepts Ollama's own syntax: `"-1"` (indefinitely), `"0"`
    /// (unload immediately), or a duration like `"5m"`/`"30s"`/`"2h"`.
    /// Invalid values are ignored (logged, no `keep_alive` sent).
    pub fn with_keep_alive(mut self, keep_alive: &str) -> Self {
        match parse_keep_alive(keep_alive) {
            Some(ka) => self.keep_alive = Some(ka),
            None => tracing::warn!("Invalid keep_alive value '{}', ignoring", keep_alive),
        }
        self
    }

    /// Override the context window size (`num_ctx`) sent with every request.
    pub fn with_num_ctx(mut self, num_ctx: u32) -> Self {
        self.num_ctx = Some(num_ctx as u64);
        self
    }

    /// Set the sampling defaults used when a request does not specify its own.
    pub fn with_sampling(
        mut self,
        temperature: Option<f32>,
        top_p: Option<f32>,
        top_k: Option<u32>,
    ) -> Self {
        self.temperature = temperature;
        self.top_p = top_p;
        self.top_k = top_k;
        self
    }

    /// Convert our generic request into an `ollama-rs` `ChatMessageRequest`.
    fn to_ollama_request(&self, request: LLMRequest) -> ChatMessageRequest {
        let mut messages: Vec<ChatMessage> = Vec::new();

        if let Some(system) = &request.system {
            messages.push(ChatMessage::system(system.clone()));
        }

        for msg in request.messages {
            let role = match msg.role {
                Role::User => MessageRole::User,
                Role::Assistant => MessageRole::Assistant,
                Role::System => MessageRole::System,
            };

            let mut text_parts = Vec::new();
            let mut tool_uses = Vec::new();
            let mut tool_results = Vec::new();
            let mut images = Vec::new();

            for block in msg.content {
                match block {
                    ContentBlock::Text { text } => text_parts.push(text),
                    ContentBlock::ToolUse { name, input, .. } => tool_uses.push((name, input)),
                    ContentBlock::ToolResult { content, .. } => tool_results.push(content),
                    ContentBlock::Image { source } => match source {
                        ImageSource::Base64 { data, .. } => images.push(Image::from_base64(data)),
                        ImageSource::Url { url } => {
                            tracing::warn!(
                                "Ollama's native chat API only accepts base64-embedded images; \
                                 skipping URL image: {}",
                                url
                            );
                        }
                    },
                    ContentBlock::Thinking { .. } => {
                        // Anthropic-specific extended-thinking block; not replayed to Ollama.
                    }
                }
            }

            if !tool_uses.is_empty() {
                // Ollama's native tool_calls carry no id; ordering alone
                // correlates them with the `tool` role messages that follow.
                let mut chat_msg = ChatMessage::new(role, text_parts.join("\n"));
                chat_msg.tool_calls = tool_uses
                    .into_iter()
                    .map(|(name, arguments)| ToolCall {
                        function: ToolCallFunction { name, arguments },
                    })
                    .collect();
                messages.push(chat_msg);
            } else if !tool_results.is_empty() {
                for content in tool_results {
                    messages.push(ChatMessage::tool(content));
                }
            } else {
                let mut chat_msg = ChatMessage::new(role, text_parts.join("\n"));
                if !images.is_empty() {
                    chat_msg = chat_msg.with_images(images);
                }
                messages.push(chat_msg);
            }
        }

        // Resolve sampling/context for THIS model: a per-model override wins,
        // else the provider-level default. Different Ollama models want
        // different tuning, so this must be keyed on the model being called.
        let ov = self.overrides_for(&request.model);

        // Request values win; the resolved per-model defaults fill the gaps.
        // The agent sets neither today, so without these the model would run at
        // Ollama's generic defaults rather than the ones it was tuned for.
        let mut options = ModelOptions::default();
        if let Some(t) = request.temperature.or(ov.temperature) {
            options = options.temperature(t);
        }
        if let Some(p) = request.top_p.or(ov.top_p) {
            options = options.top_p(p);
        }
        if let Some(k) = ov.top_k {
            options = options.top_k(k);
        }
        if let Some(seed) = request.seed.and_then(|s| i32::try_from(s).ok()) {
            options = options.seed(seed);
        }
        if let Some(stop) = request.stop.clone() {
            options = options.stop(stop);
        }
        if let Some(max_tokens) = request.max_tokens.and_then(|m| i32::try_from(m).ok()) {
            options = options.num_predict(max_tokens);
        }
        if let Some(ctx) = ov.num_ctx {
            options = options.num_ctx(ctx);
        }
        // NOTE: Ollama's ModelOptions has no frequency_penalty/presence_penalty
        // equivalent (only `repeat_penalty`, a different knob) - silently
        // dropped rather than approximated.

        let tools: Vec<ToolInfo> = request
            .tools
            .as_ref()
            .map(|ts| ts.iter().map(to_ollama_tool).collect())
            .unwrap_or_default();

        let format = request.response_format.as_ref().and_then(to_ollama_format);

        let think = request.thinking.as_ref().map(|t| match t.budget_tokens {
            0..=2_000 => ThinkType::Low,
            2_001..=8_000 => ThinkType::Medium,
            _ => ThinkType::High,
        });

        let mut ollama_request = ChatMessageRequest::new(request.model, messages).options(options);
        if !tools.is_empty() {
            ollama_request = ollama_request.tools(tools);
        }
        if let Some(format) = format {
            ollama_request = ollama_request.format(format);
        }
        if let Some(think) = think {
            ollama_request = ollama_request.think(think);
        }
        if let Some(ka) = ov.keep_alive.clone() {
            ollama_request = ollama_request.keep_alive(ka);
        }

        ollama_request
    }

    /// Convert an `ollama-rs` response into our generic `LLMResponse`,
    /// including `PerfMetrics` derived from the final-chunk timing data.
    #[allow(clippy::wrong_self_convention)]
    fn from_ollama_response(
        &self,
        response: ChatMessageResponse,
        offered_tools: &[Tool],
    ) -> LLMResponse {
        let mut content_blocks = Vec::new();

        // --- Reasoning / thinking block ---
        // Priority: explicit `message.thinking` field (native reasoning
        // models); fall back to `<think>...</think>` tags embedded in the
        // visible text (DeepSeek-R1/QwQ served through Ollama).
        let explicit_thinking = response.message.thinking.clone().filter(|t| !t.is_empty());
        let (tag_thinking, visible_text) = match &explicit_thinking {
            Some(_) => (String::new(), response.message.content.clone()),
            None => extract_think_tags(&response.message.content),
        };

        if let Some(thinking) = explicit_thinking {
            content_blocks.push(ContentBlock::Thinking { thinking });
        } else if !tag_thinking.is_empty() {
            content_blocks.push(ContentBlock::Thinking {
                thinking: tag_thinking,
            });
        }

        // Templates that never populate `tool_calls` print the call as content
        // instead. Recover it, and don't also render it as text - it is a call,
        // not something to show the user.
        let recovered = if response.message.tool_calls.is_empty() {
            tool_call_from_content(&visible_text, offered_tools)
        } else {
            None
        };

        if recovered.is_none() && !visible_text.is_empty() {
            content_blocks.push(ContentBlock::Text { text: visible_text });
        }

        for tool_call in &response.message.tool_calls {
            content_blocks.push(ContentBlock::ToolUse {
                id: uuid::Uuid::new_v4().to_string(),
                name: tool_call.function.name.clone(),
                input: tool_call.function.arguments.clone(),
            });
        }

        if let Some((name, input)) = &recovered {
            content_blocks.push(ContentBlock::ToolUse {
                id: uuid::Uuid::new_v4().to_string(),
                name: name.clone(),
                input: input.clone(),
            });
        }

        let stop_reason = if !response.message.tool_calls.is_empty() || recovered.is_some() {
            Some(StopReason::ToolUse)
        } else if response.done {
            Some(StopReason::EndTurn)
        } else {
            None
        };

        let (usage, perf_metrics) = match &response.final_data {
            Some(final_data) => (
                TokenUsage {
                    input_tokens: final_data.prompt_eval_count as u32,
                    output_tokens: final_data.eval_count as u32,
                },
                Some(perf_metrics_from_final_data(final_data)),
            ),
            None => (
                TokenUsage {
                    input_tokens: 0,
                    output_tokens: 0,
                },
                None,
            ),
        };

        LLMResponse {
            id: format!("ollama-{}", uuid::Uuid::new_v4()),
            model: response.model,
            content: content_blocks,
            stop_reason,
            usage,
            cache_metrics: None,
            perf_metrics,
        }
    }
}

#[async_trait]
impl Provider for OllamaProvider {
    async fn complete(&self, request: LLMRequest) -> Result<LLMResponse> {
        let model = request.model.clone();
        // Kept for the content-as-text tool-call fallback: a recovered call is
        // only honoured if it names a tool we actually offered.
        let offered_tools = request.tools.clone().unwrap_or_default();
        let ollama_request = self.to_ollama_request(request);

        tracing::info!(
            "Ollama API request: model={}, tools={}",
            model,
            ollama_request.tools.len()
        );

        let response = self
            .client
            .send_chat_messages(ollama_request)
            .await
            .map_err(map_ollama_error)?;

        let llm_response = self.from_ollama_response(response, &offered_tools);
        tracing::info!(
            "Ollama API response: input_tokens={}, output_tokens={}, stop_reason={:?}",
            llm_response.usage.input_tokens,
            llm_response.usage.output_tokens,
            llm_response.stop_reason
        );

        Ok(llm_response)
    }

    async fn stream(&self, request: LLMRequest) -> Result<ProviderStream> {
        let model = request.model.clone();
        let offered_tools = request.tools.clone().unwrap_or_default();
        let ollama_request = self.to_ollama_request(request);

        tracing::info!(
            "Ollama streaming request: model={}, tools={}",
            model,
            ollama_request.tools.len()
        );

        let mut chunk_stream = self
            .client
            .send_chat_messages_stream(ollama_request)
            .await
            .map_err(map_ollama_error)?;

        let message_id = format!("ollama-{}", uuid::Uuid::new_v4());
        let mut events: Vec<StreamEvent> = vec![StreamEvent::MessageStart {
            message: StreamMessage {
                id: message_id,
                model: model.clone(),
                role: Role::Assistant,
                usage: TokenUsage {
                    input_tokens: 0,
                    output_tokens: 0,
                },
            },
        }];

        let mut text_block_started = false;
        let mut streamed_text = String::new();
        let mut final_tool_calls: Vec<(String, serde_json::Value)> = Vec::new();
        let mut final_usage = TokenUsage {
            input_tokens: 0,
            output_tokens: 0,
        };
        let mut final_perf: Option<PerfMetrics> = None;
        let mut stop_reason: Option<StopReason> = None;

        {
            use futures::StreamExt as _;
            while let Some(item) = chunk_stream.next().await {
                let chunk = match item {
                    Ok(c) => c,
                    Err(()) => {
                        return Err(ProviderError::StreamError(
                            "Ollama stream terminated with an error".to_string(),
                        ));
                    }
                };

                if let Some(thinking) = chunk.message.thinking.as_ref().filter(|t| !t.is_empty()) {
                    events.push(StreamEvent::ContentBlockDelta {
                        index: 0,
                        delta: ContentDelta::ThinkingDelta {
                            thinking: thinking.clone(),
                        },
                    });
                }

                if !chunk.message.content.is_empty() {
                    streamed_text.push_str(&chunk.message.content);

                    // Stream deltas out as they arrive, as usual - but not while
                    // the content still looks like it might be a tool call the
                    // model is printing as text (qwen2.5-coder does this). Those
                    // are withheld so raw JSON never reaches the chat; if the
                    // content turns out to be prose after all, the buffered text
                    // is flushed below and nothing is lost but a little latency.
                    if !maybe_tool_call_json(&streamed_text) {
                        if !text_block_started {
                            text_block_started = true;
                            events.push(StreamEvent::ContentBlockStart {
                                index: 0,
                                content_block: ContentBlock::Text {
                                    text: String::new(),
                                },
                            });
                        }
                        events.push(StreamEvent::ContentBlockDelta {
                            index: 0,
                            delta: ContentDelta::TextDelta {
                                text: std::mem::take(&mut streamed_text),
                            },
                        });
                    }
                }

                final_tool_calls.extend(collect_tool_calls(&chunk.message.tool_calls));

                if chunk.done {
                    if let Some(final_data) = &chunk.final_data {
                        final_usage = TokenUsage {
                            input_tokens: final_data.prompt_eval_count as u32,
                            output_tokens: final_data.eval_count as u32,
                        };
                        final_perf = Some(perf_metrics_from_final_data(final_data));
                    }
                    stop_reason = Some(StopReason::EndTurn); // refined below
                }
            }
        }

        // Templates that never populate `tool_calls` print the call as content.
        // Recover it from the buffered text; on success the text is the call, so
        // it is not also streamed to the user as a message.
        if final_tool_calls.is_empty() {
            if let Some(call) = tool_call_from_content(&streamed_text, &offered_tools) {
                final_tool_calls.push(call);
                streamed_text.clear();
            }
        }

        // Whatever was withheld but turned out not to be a tool call.
        if !streamed_text.is_empty() {
            if !text_block_started {
                text_block_started = true;
                events.push(StreamEvent::ContentBlockStart {
                    index: 0,
                    content_block: ContentBlock::Text {
                        text: String::new(),
                    },
                });
            }
            events.push(StreamEvent::ContentBlockDelta {
                index: 0,
                delta: ContentDelta::TextDelta {
                    text: std::mem::take(&mut streamed_text),
                },
            });
        }

        tracing::debug!("Ollama stream done: tool_calls={}", final_tool_calls.len());

        if text_block_started {
            events.push(StreamEvent::ContentBlockStop { index: 0 });
        }

        if stop_reason.is_some() {
            stop_reason = Some(stop_reason_for(&final_tool_calls));
        }

        for (i, (name, input)) in final_tool_calls.into_iter().enumerate() {
            let index = if text_block_started { i + 1 } else { i };
            events.push(StreamEvent::ContentBlockStart {
                index,
                content_block: ContentBlock::ToolUse {
                    id: uuid::Uuid::new_v4().to_string(),
                    name,
                    input,
                },
            });
            events.push(StreamEvent::ContentBlockStop { index });
        }

        if stop_reason.is_some() {
            events.push(StreamEvent::MessageDelta {
                delta: MessageDelta {
                    stop_reason,
                    stop_sequence: None,
                },
                usage: final_usage,
                perf_metrics: final_perf,
            });
        }
        events.push(StreamEvent::MessageStop);

        let event_stream = futures::stream::iter(events.into_iter().map(Ok::<_, ProviderError>));
        Ok(Box::pin(event_stream))
    }

    fn supports_streaming(&self) -> bool {
        true
    }

    fn supports_tools(&self) -> bool {
        true
    }

    fn supports_vision(&self) -> bool {
        super::model_hints::is_vision_model(self.default_model())
    }

    fn name(&self) -> &str {
        "ollama"
    }

    fn default_model(&self) -> &str {
        self.custom_default_model.as_deref().unwrap_or("llama3.2")
    }

    fn supported_models(&self) -> Vec<String> {
        // Non-exhaustive: Ollama's model catalog is whatever the user has
        // pulled locally. This is a curated list of common picks, used only
        // as a hint; `validate_model` below always accepts any model name.
        vec![
            "llama3.2:3b".to_string(),
            "llama3.1:8b".to_string(),
            "qwen2.5-coder:7b".to_string(),
            "gemma3:12b".to_string(),
            "gemma4:26b".to_string(),
            "mistral:latest".to_string(),
            "deepseek-r1:14b".to_string(),
            "ornith:9b".to_string(),
        ]
    }

    fn validate_model(&self, _model: &str) -> bool {
        // Accept any locally-installed model name/tag.
        true
    }

    fn context_window(&self, _model: &str) -> Option<u32> {
        // `num_ctx` defaults to `DEFAULT_NUM_CTX` (see `OllamaProvider::new`)
        // rather than being unset, so this always reflects what's actually
        // requested via `to_ollama_request` - never a number Ollama itself
        // wasn't told to allocate.
        Some(
            self.num_ctx
                .map(|c| c as u32)
                .unwrap_or(DEFAULT_NUM_CTX as u32),
        )
    }

    fn calculate_cost(&self, _model: &str, _input_tokens: u32, _output_tokens: u32) -> f64 {
        // Local inference: no per-token API cost.
        0.0
    }
}

/// Pull the (name, arguments) pairs out of one streamed chunk's tool calls.
///
/// Ollama emits `tool_calls` on the chunk where the model makes the call, then
/// sends a separate terminal `done` chunk carrying none. Callers must therefore
/// accumulate across every chunk rather than reading only the `done` one.
fn collect_tool_calls(tool_calls: &[ToolCall]) -> Vec<(String, serde_json::Value)> {
    tool_calls
        .iter()
        .map(|tc| (tc.function.name.clone(), tc.function.arguments.clone()))
        .collect()
}

/// A turn ends in `ToolUse` if the model called anything across the whole stream.
fn stop_reason_for(tool_calls: &[(String, serde_json::Value)]) -> StopReason {
    if tool_calls.is_empty() {
        StopReason::EndTurn
    } else {
        StopReason::ToolUse
    }
}

/// Whether the text so far could still turn out to be a tool call printed as
/// content, and so should be withheld from the chat rather than streamed.
///
/// Only a leading `{` (or a ```json fence) qualifies. Ordinary prose therefore
/// streams token-by-token exactly as before; the buffering cost falls only on
/// content that really does look like a call.
fn maybe_tool_call_json(text: &str) -> bool {
    let t = text.trim_start();
    t.is_empty() || t.starts_with('{') || t.starts_with("```")
}

/// Recover a tool call that the model printed as text instead of returning in
/// Ollama's native `tool_calls` field.
///
/// Some Ollama chat templates - qwen2.5-coder's among them - never populate
/// `tool_calls`; the model emits `{"name": "bash", "arguments": {...}}` as its
/// message content. Without this the call is just text: nothing executes, and the
/// user sees raw JSON in the chat.
///
/// Deliberately strict, because a false positive would execute a tool the model
/// never asked for. The *entire* content must be one JSON object carrying exactly
/// a string `name` and an object `arguments` (or `parameters`), and the name must
/// match a tool that was actually offered. Prose that merely contains JSON, a
/// fenced example, or a call to an unknown tool are all left as text.
fn tool_call_from_content(content: &str, offered: &[Tool]) -> Option<(String, serde_json::Value)> {
    let trimmed = content.trim();

    // Tolerate a ```json fence, which some templates add around the object.
    let unfenced = trimmed
        .strip_prefix("```json")
        .or_else(|| trimmed.strip_prefix("```"))
        .and_then(|s| s.strip_suffix("```"))
        .map(str::trim)
        .unwrap_or(trimmed);

    if !unfenced.starts_with('{') {
        return None;
    }

    let value: serde_json::Value = serde_json::from_str(unfenced).ok()?;
    let obj = value.as_object()?;

    let name = obj.get("name")?.as_str()?;
    if !offered.iter().any(|t| t.name == name) {
        tracing::debug!("Content looks like a tool call but names no offered tool: {name}");
        return None;
    }

    // The arguments object must be present and be an object. A bare
    // `{"name": "bash"}` is not a call, and defaulting it to `{}` would invoke
    // the tool with empty input on the strength of a guess.
    let arguments = obj.get("arguments").or_else(|| obj.get("parameters"))?;
    if !arguments.is_object() {
        return None;
    }
    let arguments = arguments.clone();

    // Any other key means this is not a bare tool call - don't guess.
    if obj
        .keys()
        .any(|k| !matches!(k.as_str(), "name" | "arguments" | "parameters"))
    {
        return None;
    }

    tracing::info!(
        "Recovered a tool call the model emitted as text (its template does not \
         populate Ollama's tool_calls field): {name}"
    );
    Some((name.to_string(), arguments))
}

/// Convert our generic `Tool` (name/description/JSON-Schema) into the
/// `ToolInfo` shape `ollama-rs` sends over the wire.
fn to_ollama_tool(tool: &Tool) -> ToolInfo {
    let schema = schemars::Schema::try_from(tool.input_schema.clone()).unwrap_or_else(|e| {
        tracing::warn!(
            "Tool '{}' has an invalid JSON Schema ({e}); sending an empty schema",
            tool.name
        );
        schemars::Schema::from(serde_json::Map::new())
    });

    ToolInfo {
        tool_type: ToolType::Function,
        function: ToolFunctionInfo {
            name: tool.name.clone(),
            description: tool.description.clone(),
            parameters: schema,
        },
    }
}

/// Map our generic `response_format` (JSON mode marker or a raw JSON Schema)
/// to Ollama's `FormatType`.
fn to_ollama_format(value: &serde_json::Value) -> Option<FormatType> {
    if value.get("type").and_then(|t| t.as_str()) == Some("json_object") {
        return Some(FormatType::Json);
    }
    schemars::Schema::try_from(value.clone())
        .ok()
        .map(|schema| FormatType::StructuredJson(Box::new(JsonStructure::from(schema))))
}

/// Parse a `keep_alive` config string using Ollama's own syntax:
/// `"-1"` (indefinitely), `"0"` (unload immediately), or `"<n><unit>"`
/// with unit `s`/`m`/`h` (e.g. `"5m"`).
fn parse_keep_alive(s: &str) -> Option<KeepAlive> {
    match s {
        "-1" => Some(KeepAlive::Indefinitely),
        "0" => Some(KeepAlive::UnloadOnCompletion),
        other => {
            let last_char = other.chars().next_back()?;
            let unit = match last_char {
                's' => TimeUnit::Seconds,
                'm' => TimeUnit::Minutes,
                'h' => TimeUnit::Hours,
                _ => return None,
            };
            let time = other[..other.len() - last_char.len_utf8()]
                .parse::<u64>()
                .ok()?;
            Some(KeepAlive::Until { time, unit })
        }
    }
}

/// Convert Ollama's nanosecond-resolution timing data into `PerfMetrics`
/// (millisecond resolution, plus derived warm/cold-start flag).
fn perf_metrics_from_final_data(final_data: &ChatMessageFinalResponseData) -> PerfMetrics {
    const NS_PER_MS: u64 = 1_000_000;
    PerfMetrics {
        load_duration_ms: Some(final_data.load_duration / NS_PER_MS),
        prompt_eval_duration_ms: Some(final_data.prompt_eval_duration / NS_PER_MS),
        eval_duration_ms: Some(final_data.eval_duration / NS_PER_MS),
        total_duration_ms: Some(final_data.total_duration / NS_PER_MS),
        model_was_loaded: Some(final_data.load_duration == 0),
    }
}

/// Map `ollama-rs`'s error type to our provider-agnostic `ProviderError`.
///
/// See the module-level doc comment for why `OllamaError::ReqwestError`
/// cannot become `ProviderError::HttpError` here.
fn map_ollama_error(err: OllamaError) -> ProviderError {
    match err {
        OllamaError::JsonError(e) => ProviderError::JsonError(e),
        OllamaError::ToolCallError(e) => ProviderError::InvalidRequest(e.to_string()),
        OllamaError::InternalError(inner) => ProviderError::ApiError {
            status: 0,
            message: inner.message,
            error_type: None,
        },
        OllamaError::ReqwestError(e) => ProviderError::ApiError {
            status: 0,
            message: format!("Ollama network error: {e}"),
            error_type: Some("network_error".to_string()),
        },
        OllamaError::Other(msg) => {
            if msg.to_lowercase().contains("not found") {
                ProviderError::ModelNotFound(msg)
            } else {
                ProviderError::ApiError {
                    status: 0,
                    message: msg,
                    error_type: None,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ollama_provider_creation() {
        let provider = OllamaProvider::default_local();
        assert_eq!(provider.name(), "ollama");
        assert_eq!(provider.default_model(), "llama3.2");
    }

    #[test]
    fn test_with_default_model() {
        let provider =
            OllamaProvider::default_local().with_default_model("qwen2.5-coder:7b".to_string());
        assert_eq!(provider.default_model(), "qwen2.5-coder:7b");
    }

    #[test]
    fn per_model_override_wins_over_provider_default_for_that_model() {
        let mut per_model = std::collections::HashMap::new();
        per_model.insert(
            "ornith:9b".to_string(),
            ModelOverrides::from_config(Some(0.6), Some(0.95), Some(20), None, None),
        );
        let provider = OllamaProvider::default_local()
            .with_sampling(Some(0.2), Some(0.5), Some(40)) // provider-level fallback
            .with_per_model(per_model);

        // ornith gets its own tuning...
        let ornith = provider.overrides_for("ornith:9b");
        assert_eq!(ornith.temperature, Some(0.6));
        assert_eq!(ornith.top_p, Some(0.95));
        assert_eq!(ornith.top_k, Some(20));

        // ...while every other model still gets the provider-level defaults,
        // so tuning ornith can no longer degrade qwen.
        let qwen = provider.overrides_for("qwen2.5-coder:7b");
        assert_eq!(qwen.temperature, Some(0.2));
        assert_eq!(qwen.top_p, Some(0.5));
        assert_eq!(qwen.top_k, Some(40));
    }

    #[test]
    fn per_model_override_falls_back_field_by_field() {
        let mut per_model = std::collections::HashMap::new();
        // Only temperature is set per-model; the rest must fall back.
        per_model.insert(
            "ornith:9b".to_string(),
            ModelOverrides::from_config(Some(0.6), None, None, None, None),
        );
        let provider = OllamaProvider::default_local()
            .with_sampling(Some(0.2), Some(0.5), Some(40))
            .with_per_model(per_model);

        let ov = provider.overrides_for("ornith:9b");
        assert_eq!(ov.temperature, Some(0.6), "per-model value wins");
        assert_eq!(ov.top_p, Some(0.5), "unset per-model field falls back");
        assert_eq!(ov.top_k, Some(40), "unset per-model field falls back");
    }

    #[test]
    fn test_invalid_host_falls_back_to_default() {
        // Not a valid URL - must not panic, falls back to the default host.
        let provider = OllamaProvider::new("not a url");
        assert_eq!(provider.name(), "ollama");
    }

    #[test]
    fn test_validate_model_always_true() {
        let provider = OllamaProvider::default_local();
        assert!(provider.validate_model("anything-the-user-pulled:latest"));
    }

    #[test]
    fn test_supported_models_includes_gemma4() {
        let provider = OllamaProvider::default_local();
        assert!(provider
            .supported_models()
            .contains(&"gemma4:26b".to_string()));
    }

    #[test]
    fn test_supported_models_includes_ornith() {
        let provider = OllamaProvider::default_local();
        let models = provider.supported_models();
        assert!(models.contains(&"ornith:9b".to_string()));
    }

    #[test]
    fn test_context_window_default_and_custom() {
        let provider = OllamaProvider::default_local();
        assert_eq!(provider.context_window("llama3.2"), Some(8_192));

        let custom = OllamaProvider::default_local().with_num_ctx(32_768);
        assert_eq!(custom.context_window("llama3.2"), Some(32_768));
    }

    #[test]
    fn test_calculate_cost_is_always_zero() {
        let provider = OllamaProvider::default_local();
        assert_eq!(provider.calculate_cost("llama3.2", 10_000, 10_000), 0.0);
    }

    #[test]
    fn test_supports_vision_detection() {
        let vision = OllamaProvider::default_local().with_default_model("llava:13b".to_string());
        assert!(vision.supports_vision());

        let plain = OllamaProvider::default_local().with_default_model("llama3.2:8b".to_string());
        assert!(!plain.supports_vision());
    }

    #[test]
    fn test_parse_keep_alive() {
        assert_eq!(parse_keep_alive("-1"), Some(KeepAlive::Indefinitely));
        assert_eq!(parse_keep_alive("0"), Some(KeepAlive::UnloadOnCompletion));
        assert_eq!(
            parse_keep_alive("5m"),
            Some(KeepAlive::Until {
                time: 5,
                unit: TimeUnit::Minutes
            })
        );
        assert_eq!(parse_keep_alive("garbage"), None);
    }

    #[test]
    fn test_perf_metrics_from_final_data() {
        let final_data = ChatMessageFinalResponseData {
            total_duration: 5_000_000_000,
            load_duration: 0,
            prompt_eval_count: 50,
            prompt_eval_duration: 500_000_000,
            eval_count: 200,
            eval_duration: 4_000_000_000,
        };
        let perf = perf_metrics_from_final_data(&final_data);
        assert_eq!(perf.total_duration_ms, Some(5_000));
        assert_eq!(perf.load_duration_ms, Some(0));
        assert_eq!(perf.prompt_eval_duration_ms, Some(500));
        assert_eq!(perf.eval_duration_ms, Some(4_000));
        assert_eq!(perf.model_was_loaded, Some(true));
        assert_eq!(perf.tokens_per_second(200), Some(50.0));
    }

    #[test]
    fn test_map_ollama_error_not_found() {
        let err = map_ollama_error(OllamaError::Other(
            "model \"bogus\" not found, try pulling it first".to_string(),
        ));
        assert!(matches!(err, ProviderError::ModelNotFound(_)));
    }

    #[test]
    fn test_to_ollama_request_maps_common_fields() {
        let provider = OllamaProvider::default_local();
        let request = LLMRequest::new("llama3.2", vec![Message::user("hi")])
            .with_system("be terse")
            .with_temperature(0.5)
            .with_top_p(0.9)
            .with_seed(42)
            .with_stop(vec!["STOP".to_string()])
            .with_max_tokens(100);

        let ollama_request = provider.to_ollama_request(request);
        assert_eq!(ollama_request.model_name, "llama3.2");
        // system message + user message
        assert_eq!(ollama_request.messages.len(), 2);
        assert_eq!(ollama_request.messages[0].role, MessageRole::System);
        assert_eq!(ollama_request.messages[1].role, MessageRole::User);
    }

    fn mock_response(message: ChatMessage, done: bool) -> ChatMessageResponse {
        ChatMessageResponse {
            model: "llama3.2".to_string(),
            created_at: "2026-01-01T00:00:00Z".to_string(),
            message,
            logprobs: None,
            done,
            final_data: None,
        }
    }

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

    /// qwen2.5-coder's Ollama template never populates `tool_calls` - it prints
    /// the call as message content. Without recovering it, nothing executes and
    /// the user just sees raw JSON in the chat.
    #[test]
    fn tool_call_printed_as_content_is_recovered() {
        let tools = [bash_tool()];
        let content =
            "{\n  \"name\": \"bash\",\n  \"arguments\": {\n    \"command\": \"ls -la\"\n  }\n}";

        let (name, args) =
            tool_call_from_content(content, &tools).expect("call should be recovered");
        assert_eq!(name, "bash");
        assert_eq!(args["command"], "ls -la");
    }

    #[test]
    fn tool_call_in_a_json_fence_is_recovered() {
        let tools = [bash_tool()];
        let content = "```json\n{\"name\": \"bash\", \"arguments\": {\"command\": \"ls\"}}\n```";
        let (name, _) = tool_call_from_content(content, &tools).expect("recovered");
        assert_eq!(name, "bash");
    }

    /// The recovery must never fire on content that merely *contains* JSON, or
    /// Crustly would execute a tool the model never asked for.
    #[test]
    fn prose_is_never_mistaken_for_a_tool_call() {
        let tools = [bash_tool()];
        for content in [
            "Here is an example: {\"name\": \"bash\", \"arguments\": {}}",
            "I will run ls for you.",
            "{\"name\": \"rm_rf\", \"arguments\": {}}", // not an offered tool
            "{\"name\": \"bash\"}",                     // no arguments object
            "{\"name\": \"bash\", \"arguments\": {}, \"note\": \"extra\"}", // unexpected key
            "{\"arguments\": {\"command\": \"ls\"}}",   // no name
            "{}",
            "",
        ] {
            assert!(
                tool_call_from_content(content, &tools).is_none(),
                "must not be treated as a tool call: {content:?}"
            );
        }
    }

    /// Prose must still stream token-by-token; only content that might be a
    /// printed tool call is withheld.
    #[test]
    fn only_json_like_content_is_withheld_from_streaming() {
        assert!(maybe_tool_call_json(""));
        assert!(maybe_tool_call_json("{"));
        assert!(maybe_tool_call_json("  {\"name\""));
        assert!(maybe_tool_call_json("```json"));
        assert!(!maybe_tool_call_json("Here are the files"));
        assert!(!maybe_tool_call_json("I'll run ls."));
    }

    /// End to end through the non-streaming path: the recovered call must surface
    /// as a ToolUse block with StopReason::ToolUse, and the JSON must NOT also be
    /// shown to the user as text.
    #[test]
    fn recovered_tool_call_becomes_a_tool_use_block() {
        let provider = OllamaProvider::default_local();
        let response = mock_response(
            ChatMessage::assistant(
                "{\"name\": \"bash\", \"arguments\": {\"command\": \"ls -la\"}}".to_string(),
            ),
            true,
        );

        let llm = provider.from_ollama_response(response, &[bash_tool()]);

        assert_eq!(llm.stop_reason, Some(StopReason::ToolUse));
        let tool_uses: Vec<_> = llm
            .content
            .iter()
            .filter(|b| matches!(b, ContentBlock::ToolUse { .. }))
            .collect();
        assert_eq!(tool_uses.len(), 1, "expected one ToolUse block");
        assert!(
            !llm.content
                .iter()
                .any(|b| matches!(b, ContentBlock::Text { .. })),
            "the JSON is the call - it must not also be rendered as a message"
        );
    }

    #[test]
    fn from_ollama_response_plain_text_with_final_data() {
        let provider = OllamaProvider::default_local();
        let mut response = mock_response(ChatMessage::assistant("hello there".to_string()), true);
        response.final_data = Some(ChatMessageFinalResponseData {
            total_duration: 2_000_000_000,
            load_duration: 0,
            prompt_eval_count: 10,
            prompt_eval_duration: 100_000_000,
            eval_count: 20,
            eval_duration: 1_000_000_000,
        });

        let llm_response = provider.from_ollama_response(response, &[]);
        assert_eq!(llm_response.model, "llama3.2");
        assert_eq!(llm_response.stop_reason, Some(StopReason::EndTurn));
        assert_eq!(llm_response.usage.input_tokens, 10);
        assert_eq!(llm_response.usage.output_tokens, 20);
        assert!(llm_response.perf_metrics.is_some());
        assert_eq!(llm_response.content.len(), 1);
        assert!(matches!(
            &llm_response.content[0],
            ContentBlock::Text { text } if text == "hello there"
        ));
    }

    #[test]
    fn from_ollama_response_without_final_data_has_zero_usage_and_no_perf() {
        let provider = OllamaProvider::default_local();
        // Mid-stream chunk: done=false, no final_data yet.
        let response = mock_response(ChatMessage::assistant("partial".to_string()), false);

        let llm_response = provider.from_ollama_response(response, &[]);
        assert_eq!(llm_response.stop_reason, None);
        assert_eq!(llm_response.usage.input_tokens, 0);
        assert_eq!(llm_response.usage.output_tokens, 0);
        assert!(llm_response.perf_metrics.is_none());
    }

    #[test]
    fn from_ollama_response_extracts_tool_calls() {
        let provider = OllamaProvider::default_local();
        let mut message = ChatMessage::assistant(String::new());
        message.tool_calls = vec![ToolCall {
            function: ToolCallFunction {
                name: "read_file".to_string(),
                arguments: serde_json::json!({"path": "src/main.rs"}),
            },
        }];
        let response = mock_response(message, true);

        let llm_response = provider.from_ollama_response(response, &[]);
        assert_eq!(llm_response.stop_reason, Some(StopReason::ToolUse));
        assert_eq!(llm_response.content.len(), 1);
        match &llm_response.content[0] {
            ContentBlock::ToolUse { name, input, .. } => {
                assert_eq!(name, "read_file");
                assert_eq!(input["path"], "src/main.rs");
            }
            other => panic!("expected ToolUse block, got {other:?}"),
        }
    }

    /// Regression: Ollama streams tool_calls on a `done=false` chunk and then a
    /// terminal `done=true` chunk with none. Reading only the `done` chunk lost
    /// the call entirely and ended the turn as EndTurn, so no tool ever ran.
    #[test]
    fn streamed_tool_calls_arrive_before_the_done_chunk() {
        let call_chunk = vec![ToolCall {
            function: ToolCallFunction {
                name: "bash".to_string(),
                arguments: serde_json::json!({"command": "ls -la"}),
            },
        }];
        let done_chunk: Vec<ToolCall> = vec![];

        let mut accumulated = Vec::new();
        accumulated.extend(collect_tool_calls(&call_chunk));
        accumulated.extend(collect_tool_calls(&done_chunk));

        assert_eq!(accumulated.len(), 1, "tool call from mid-stream chunk kept");
        assert_eq!(accumulated[0].0, "bash");
        assert_eq!(accumulated[0].1["command"], "ls -la");
        assert_eq!(stop_reason_for(&accumulated), StopReason::ToolUse);
    }

    #[test]
    fn stream_without_tool_calls_ends_the_turn() {
        assert_eq!(stop_reason_for(&[]), StopReason::EndTurn);
    }

    /// End-to-end against a real Ollama daemon; ignored by default because it
    /// needs `ollama serve` and a tool-capable model pulled locally.
    ///
    ///   cargo test --features ollama -- --ignored streamed_tool_call_reaches_caller
    #[tokio::test]
    #[ignore = "requires a running Ollama with a tool-capable model"]
    async fn streamed_tool_call_reaches_caller() {
        use futures::StreamExt as _;

        let model = std::env::var("OLLAMA_MODEL").unwrap_or_else(|_| "ornith:9b".to_string());
        let provider = OllamaProvider::default_local().with_default_model(model.clone());

        let request = LLMRequest::new(
            &model,
            vec![Message::user("List the files here. Use the bash tool.")],
        )
        .with_tools(vec![Tool {
            name: "bash".to_string(),
            description: "Run a shell command".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {"command": {"type": "string"}},
                "required": ["command"],
            }),
        }]);

        let mut stream = provider.stream(request).await.expect("stream starts");

        let mut tool_uses = Vec::new();
        let mut stop_reason = None;
        while let Some(event) = stream.next().await {
            match event.expect("no stream error") {
                StreamEvent::ContentBlockStart {
                    content_block: ContentBlock::ToolUse { name, input, .. },
                    ..
                } => tool_uses.push((name, input)),
                StreamEvent::MessageDelta { delta, .. } => stop_reason = delta.stop_reason,
                _ => {}
            }
        }

        assert!(
            !tool_uses.is_empty(),
            "model asked to call a tool but no ToolUse block surfaced (tool calls \
             arrive on a pre-`done` chunk; regression if dropped)"
        );
        assert_eq!(tool_uses[0].0, "bash");
        assert_eq!(stop_reason, Some(StopReason::ToolUse));
    }

    #[test]
    fn from_ollama_response_uses_explicit_thinking_field() {
        let provider = OllamaProvider::default_local();
        let mut message = ChatMessage::assistant("the answer is 42".to_string());
        message.thinking = Some("reasoning about the question".to_string());
        let response = mock_response(message, true);

        let llm_response = provider.from_ollama_response(response, &[]);
        assert_eq!(llm_response.content.len(), 2);
        assert!(matches!(
            &llm_response.content[0],
            ContentBlock::Thinking { thinking } if thinking == "reasoning about the question"
        ));
        assert!(matches!(
            &llm_response.content[1],
            ContentBlock::Text { text } if text == "the answer is 42"
        ));
    }

    #[test]
    fn from_ollama_response_falls_back_to_think_tags() {
        let provider = OllamaProvider::default_local();
        // No explicit `thinking` field - reasoning embedded as <think> tags
        // in the visible text (DeepSeek-R1/QwQ style models via Ollama).
        let message =
            ChatMessage::assistant("<think>let me work this out</think>final answer".to_string());
        let response = mock_response(message, true);

        let llm_response = provider.from_ollama_response(response, &[]);
        assert!(matches!(
            &llm_response.content[0],
            ContentBlock::Thinking { thinking } if thinking == "let me work this out"
        ));
        assert!(matches!(
            &llm_response.content[1],
            ContentBlock::Text { text } if text == "final answer"
        ));
    }

    #[test]
    fn to_ollama_tool_converts_valid_schema() {
        let tool = Tool {
            name: "read_file".to_string(),
            description: "Reads a file".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": { "path": { "type": "string" } }
            }),
        };

        let info = to_ollama_tool(&tool);
        assert_eq!(info.function.name, "read_file");
        assert_eq!(info.function.description, "Reads a file");
    }

    #[test]
    fn to_ollama_tool_falls_back_on_invalid_schema() {
        // A bare string is neither a JSON object nor a bool, so
        // `Schema::try_from` fails and we fall back to an empty schema
        // instead of propagating the error.
        let tool = Tool {
            name: "broken".to_string(),
            description: "d".to_string(),
            input_schema: serde_json::json!("not a schema"),
        };

        let info = to_ollama_tool(&tool);
        assert_eq!(info.function.name, "broken");
    }

    #[test]
    fn to_ollama_format_json_object_marker() {
        let value = serde_json::json!({"type": "json_object"});
        assert!(matches!(to_ollama_format(&value), Some(FormatType::Json)));
    }

    #[test]
    fn to_ollama_format_structured_schema() {
        let value = serde_json::json!({
            "type": "object",
            "properties": { "answer": { "type": "string" } }
        });
        assert!(matches!(
            to_ollama_format(&value),
            Some(FormatType::StructuredJson(_))
        ));
    }

    #[test]
    fn to_ollama_request_maps_tool_messages() {
        let provider = OllamaProvider::default_local();
        let request = LLMRequest::new(
            "llama3.2",
            vec![
                Message {
                    role: Role::Assistant,
                    content: vec![ContentBlock::ToolUse {
                        id: "call_1".to_string(),
                        name: "read_file".to_string(),
                        input: serde_json::json!({"path": "a.rs"}),
                    }],
                },
                Message {
                    role: Role::User,
                    content: vec![ContentBlock::ToolResult {
                        tool_use_id: "call_1".to_string(),
                        content: "file contents".to_string(),
                        is_error: None,
                    }],
                },
            ],
        );

        let ollama_request = provider.to_ollama_request(request);
        assert_eq!(ollama_request.messages.len(), 2);
        assert_eq!(ollama_request.messages[0].tool_calls.len(), 1);
        assert_eq!(
            ollama_request.messages[0].tool_calls[0].function.name,
            "read_file"
        );
        assert_eq!(ollama_request.messages[1].role, MessageRole::Tool);
        assert_eq!(ollama_request.messages[1].content, "file contents");
    }

    #[test]
    fn to_ollama_request_maps_thinking_and_response_format() {
        let provider = OllamaProvider::default_local();
        let request = LLMRequest::new("llama3.2", vec![Message::user("hi")])
            .with_thinking(4_000)
            .with_response_format(serde_json::json!({"type": "json_object"}));

        let ollama_request = provider.to_ollama_request(request);
        assert!(matches!(ollama_request.think, Some(ThinkType::Medium)));
        assert!(matches!(ollama_request.format, Some(FormatType::Json)));
    }

    #[test]
    fn to_ollama_request_embeds_base64_image() {
        let provider = OllamaProvider::default_local();
        let request = LLMRequest::new(
            "llava:13b",
            vec![Message {
                role: Role::User,
                content: vec![
                    ContentBlock::Text {
                        text: "what is this?".to_string(),
                    },
                    ContentBlock::Image {
                        source: ImageSource::Base64 {
                            media_type: "image/png".to_string(),
                            data: "aGVsbG8=".to_string(),
                        },
                    },
                ],
            }],
        );

        let ollama_request = provider.to_ollama_request(request);
        let images = ollama_request.messages[0]
            .images
            .as_ref()
            .expect("image should be attached");
        assert_eq!(images.len(), 1);
        assert_eq!(images[0].to_base64(), "aGVsbG8=");
    }
}
