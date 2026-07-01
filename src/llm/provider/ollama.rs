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

/// Ollama provider using the native `/api/chat` protocol.
#[derive(Clone)]
pub struct OllamaProvider {
    client: Ollama,
    custom_default_model: Option<String>,
    keep_alive: Option<KeepAlive>,
    num_ctx: Option<u64>,
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
            num_ctx: None,
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

        let mut options = ModelOptions::default();
        if let Some(t) = request.temperature {
            options = options.temperature(t);
        }
        if let Some(p) = request.top_p {
            options = options.top_p(p);
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
        if let Some(ctx) = self.num_ctx {
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

        let mut ollama_request =
            ChatMessageRequest::new(request.model, messages).options(options);
        if !tools.is_empty() {
            ollama_request = ollama_request.tools(tools);
        }
        if let Some(format) = format {
            ollama_request = ollama_request.format(format);
        }
        if let Some(think) = think {
            ollama_request = ollama_request.think(think);
        }
        if let Some(ka) = self.keep_alive.clone() {
            ollama_request = ollama_request.keep_alive(ka);
        }

        ollama_request
    }

    /// Convert an `ollama-rs` response into our generic `LLMResponse`,
    /// including `PerfMetrics` derived from the final-chunk timing data.
    fn from_ollama_response(&self, response: ChatMessageResponse) -> LLMResponse {
        let mut content_blocks = Vec::new();

        // --- Reasoning / thinking block ---
        // Priority: explicit `message.thinking` field (native reasoning
        // models); fall back to `<think>...</think>` tags embedded in the
        // visible text (DeepSeek-R1/QwQ served through Ollama).
        let explicit_thinking = response
            .message
            .thinking
            .clone()
            .filter(|t| !t.is_empty());
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

        if !visible_text.is_empty() {
            content_blocks.push(ContentBlock::Text { text: visible_text });
        }

        for tool_call in &response.message.tool_calls {
            content_blocks.push(ContentBlock::ToolUse {
                id: uuid::Uuid::new_v4().to_string(),
                name: tool_call.function.name.clone(),
                input: tool_call.function.arguments.clone(),
            });
        }

        let stop_reason = if !response.message.tool_calls.is_empty() {
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

        let llm_response = self.from_ollama_response(response);
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
        let ollama_request = self.to_ollama_request(request);

        tracing::info!("Ollama streaming request: model={}", model);

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
                            text: chunk.message.content.clone(),
                        },
                    });
                }

                if chunk.done {
                    if !chunk.message.tool_calls.is_empty() {
                        stop_reason = Some(StopReason::ToolUse);
                        final_tool_calls = chunk
                            .message
                            .tool_calls
                            .iter()
                            .map(|tc| (tc.function.name.clone(), tc.function.arguments.clone()))
                            .collect();
                    } else {
                        stop_reason = Some(StopReason::EndTurn);
                    }

                    if let Some(final_data) = &chunk.final_data {
                        final_usage = TokenUsage {
                            input_tokens: final_data.prompt_eval_count as u32,
                            output_tokens: final_data.eval_count as u32,
                        };
                        final_perf = Some(perf_metrics_from_final_data(final_data));
                    }
                }
            }
        }

        if text_block_started {
            events.push(StreamEvent::ContentBlockStop { index: 0 });
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
            });
        }
        // `perf_metrics` from `final_perf` isn't threaded onto `StreamEvent`
        // today (no slot for it); non-streaming `complete()` is the primary
        // path for surfacing PerfMetrics in the TUI (see agent/service.rs).
        let _ = final_perf;
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
            "mistral:latest".to_string(),
            "deepseek-r1:14b".to_string(),
        ]
    }

    fn validate_model(&self, _model: &str) -> bool {
        // Accept any locally-installed model name/tag.
        true
    }

    fn context_window(&self, _model: &str) -> Option<u32> {
        // If the caller configured a custom num_ctx, that's the actual
        // running context window regardless of model. Otherwise fall back
        // to a conservative default (most current local models support at
        // least 8K), matching OpenAIProvider::local()'s behavior.
        Some(self.num_ctx.map(|c| c as u32).unwrap_or(8_192))
    }

    fn calculate_cost(&self, _model: &str, _input_tokens: u32, _output_tokens: u32) -> f64 {
        // Local inference: no per-token API cost.
        0.0
    }
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
}
