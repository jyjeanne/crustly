//! OpenAI Provider Implementation
//!
//! Implements the Provider trait for OpenAI's GPT models.
//!
//! ## Supported Models
//! - gpt-4-turbo-preview
//! - gpt-4
//! - gpt-4-32k
//! - gpt-3.5-turbo
//! - gpt-3.5-turbo-16k
//!
//! ## Compatibility
//! This implementation also works with OpenAI-compatible APIs:
//! - Local LLMs via LM Studio (http://localhost:1234/v1)
//! - Ollama with OpenAI compatibility (http://localhost:11434/v1)
//! - LocalAI and other compatible APIs

use super::error::{ProviderError, Result};
use super::r#trait::{Provider, ProviderStream};
use super::types::*;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

const DEFAULT_OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(120);
const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
const DEFAULT_POOL_IDLE_TIMEOUT: Duration = Duration::from_secs(90);

/// No real model emits more than a handful of parallel tool calls in one
/// response, so a streamed tool-call delta's `index` (read straight off the
/// wire with no upper bound from the, possibly third-party/local, endpoint)
/// is capped here.
const MAX_TOOL_CALL_INDEX: usize = 128;

/// Whether a streamed tool-call delta's index is safe to use for growing
/// `tool_call_builders`. Guards against a malformed/hostile response (e.g.
/// `"index": 500000000`) forcing a huge allocation - fatal here since the
/// crate is built with `panic = "abort"`.
fn tool_call_index_in_bounds(idx: usize) -> bool {
    idx <= MAX_TOOL_CALL_INDEX
}

/// How the API key is sent. OpenAI (and every OpenAI-compatible local
/// server this provider also drives) expects `Authorization: Bearer <key>`.
/// Azure OpenAI's REST API rejects that for key-based auth - it requires a
/// plain `api-key: <key>` header instead (`Bearer` is only valid there for
/// Entra ID/AAD tokens, which this provider doesn't implement) - so
/// `AzureOpenAIProvider` selects the other variant via
/// [`OpenAIProvider::with_api_key_header`].
#[derive(Clone, Copy, PartialEq, Eq)]
enum AuthStyle {
    Bearer,
    ApiKeyHeader,
}

/// OpenAI provider for GPT models
#[derive(Clone)]
pub struct OpenAIProvider {
    api_key: String,
    base_url: String,
    client: Client,
    custom_default_model: Option<String>,
    auth_style: AuthStyle,
}

impl OpenAIProvider {
    /// Create a new OpenAI provider with official API
    pub fn new(api_key: String) -> Self {
        let client = Client::builder()
            .timeout(DEFAULT_TIMEOUT)
            .connect_timeout(DEFAULT_CONNECT_TIMEOUT)
            .pool_idle_timeout(DEFAULT_POOL_IDLE_TIMEOUT)
            .pool_max_idle_per_host(2)
            .build()
            .expect("Failed to create HTTP client");

        Self {
            api_key,
            base_url: DEFAULT_OPENAI_API_URL.to_string(),
            client,
            custom_default_model: None,
            auth_style: AuthStyle::Bearer,
        }
    }

    /// Create provider for local LLM (LM Studio, Ollama, etc.)
    pub fn local(base_url: String) -> Self {
        let client = Client::builder()
            .timeout(DEFAULT_TIMEOUT)
            .connect_timeout(DEFAULT_CONNECT_TIMEOUT)
            .pool_idle_timeout(DEFAULT_POOL_IDLE_TIMEOUT)
            .pool_max_idle_per_host(2)
            .build()
            .expect("Failed to create HTTP client");

        Self {
            api_key: "not-needed".to_string(),
            base_url,
            client,
            custom_default_model: None,
            auth_style: AuthStyle::Bearer,
        }
    }

    /// Create with custom base URL
    pub fn with_base_url(api_key: String, base_url: String) -> Self {
        let client = Client::builder()
            .timeout(DEFAULT_TIMEOUT)
            .connect_timeout(DEFAULT_CONNECT_TIMEOUT)
            .pool_idle_timeout(DEFAULT_POOL_IDLE_TIMEOUT)
            .pool_max_idle_per_host(2)
            .build()
            .expect("Failed to create HTTP client");

        Self {
            api_key,
            base_url,
            client,
            custom_default_model: None,
            auth_style: AuthStyle::Bearer,
        }
    }

    /// Set custom default model (useful for local LLMs with specific model names)
    pub fn with_default_model(mut self, model: String) -> Self {
        self.custom_default_model = Some(model);
        self
    }

    /// Send the API key as a plain `api-key` header instead of
    /// `Authorization: Bearer`. Azure OpenAI's REST API requires this for
    /// key-based auth - `Bearer` is only valid there for Entra ID/AAD
    /// tokens - so every request made with the default `Bearer` style
    /// failed with 401 Unauthorized for any Azure OpenAI user, with no
    /// working code path. Used by `AzureOpenAIProvider`.
    pub(crate) fn with_api_key_header(mut self) -> Self {
        self.auth_style = AuthStyle::ApiKeyHeader;
        self
    }

    /// Build request headers.
    ///
    /// The API key is user/config-supplied and commonly picks up a trailing
    /// newline or other whitespace (e.g. loaded from a file/env var);
    /// `HeaderValue::parse` rejects any byte outside the printable-ASCII
    /// header range, so an `.expect()` here used to crash the whole process
    /// on the very first request. Trim first and return a proper error for
    /// anything still invalid instead of panicking.
    fn headers(&self) -> Result<reqwest::header::HeaderMap> {
        let mut headers = reqwest::header::HeaderMap::new();

        // Only add authorization if not using local
        if self.api_key != "not-needed" {
            match self.auth_style {
                AuthStyle::Bearer => {
                    headers.insert(
                        reqwest::header::AUTHORIZATION,
                        format!("Bearer {}", self.api_key.trim())
                            .parse()
                            .map_err(|_| ProviderError::InvalidApiKey)?,
                    );
                }
                AuthStyle::ApiKeyHeader => {
                    headers.insert(
                        "api-key",
                        self.api_key
                            .trim()
                            .parse()
                            .map_err(|_| ProviderError::InvalidApiKey)?,
                    );
                }
            }
        }

        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json"
                .parse()
                .expect("static content-type string is always a valid header value"),
        );

        Ok(headers)
    }

    /// Convert our generic request to OpenAI-specific format
    fn to_openai_request(&self, request: LLMRequest) -> OpenAIRequest {
        let mut messages = Vec::new();

        // Add system message if present
        if let Some(system) = request.system {
            messages.push(OpenAIMessage {
                role: "system".to_string(),
                content: Some(system),
                tool_calls: None,
                tool_call_id: None,
                reasoning_content: None,
            });
        }

        // Add conversation messages
        for msg in request.messages {
            let role = match msg.role {
                Role::User => "user",
                Role::Assistant => "assistant",
                Role::System => "system",
            };

            // Separate content blocks by type
            let mut text_parts = Vec::new();
            let mut tool_uses = Vec::new();
            let mut tool_results = Vec::new();

            for block in msg.content {
                match block {
                    ContentBlock::Text { text } => {
                        text_parts.push(text);
                    }
                    ContentBlock::ToolUse { id, name, input } => {
                        tool_uses.push((id, name, input));
                    }
                    ContentBlock::ToolResult {
                        tool_use_id,
                        content,
                        ..
                    } => {
                        tool_results.push((tool_use_id, content));
                    }
                    ContentBlock::Image { .. } => {
                        // Skip images for now (OpenAI needs special handling)
                        tracing::warn!("Image content blocks not yet supported for OpenAI");
                    }
                    ContentBlock::Thinking { .. } => {
                        // Thinking blocks are Anthropic-specific; skip for OpenAI
                    }
                }
            }

            // Handle assistant messages with tool calls
            if !tool_uses.is_empty() {
                let openai_tool_calls = tool_uses
                    .into_iter()
                    .map(|(id, name, input)| OpenAIToolCall {
                        id,
                        r#type: "function".to_string(),
                        function: OpenAIFunctionCall {
                            name,
                            arguments: serde_json::to_string(&input).unwrap_or_default(),
                        },
                    })
                    .collect();

                let content_str = if text_parts.is_empty() {
                    None
                } else {
                    Some(text_parts.join("\n"))
                };

                messages.push(OpenAIMessage {
                    role: role.to_string(),
                    content: content_str,
                    tool_calls: Some(openai_tool_calls),
                    tool_call_id: None,
                    reasoning_content: None,
                });
            }
            // Handle tool result messages
            else if !tool_results.is_empty() {
                for (tool_use_id, content) in tool_results {
                    messages.push(OpenAIMessage {
                        role: "tool".to_string(),
                        content: Some(content),
                        tool_calls: None,
                        tool_call_id: Some(tool_use_id),
                        reasoning_content: None,
                    });
                }
                // Text sent with the results (notes saved about the files the
                // tools used) follows them as a user message; `tool` messages
                // carry only results.
                if !text_parts.is_empty() {
                    messages.push(OpenAIMessage {
                        role: "user".to_string(),
                        content: Some(text_parts.join("\n")),
                        tool_calls: None,
                        tool_call_id: None,
                        reasoning_content: None,
                    });
                }
            }
            // Handle regular text messages
            else {
                let content_str = if text_parts.is_empty() {
                    Some(String::new())
                } else {
                    Some(text_parts.join("\n"))
                };

                messages.push(OpenAIMessage {
                    role: role.to_string(),
                    content: content_str,
                    tool_calls: None,
                    tool_call_id: None,
                    reasoning_content: None,
                });
            }
        }

        // Convert tools to OpenAI format
        let tools = request.tools.map(|tools| {
            tools
                .iter()
                .map(|tool| OpenAITool {
                    r#type: "function".to_string(),
                    function: OpenAIFunction {
                        name: tool.name.clone(),
                        description: tool.description.clone(),
                        parameters: tool.input_schema.clone(),
                    },
                })
                .collect()
        });

        // Set tool_choice to "auto" whenever tools are present so that models
        // (including Ollama-hosted ones) reliably invoke tools instead of
        // responding with plain text.
        let tool_choice = tools.as_ref().map(|_| "auto".to_string());

        // Map Anthropic-style thinking config to OpenAI/Ollama reasoning_effort.
        // This enables the reasoning trace on thinking-capable models such as
        // DeepSeek-R1, Qwen3, and GPT-OSS served through Ollama.
        let reasoning_effort = request.thinking.as_ref().map(|t| match t.budget_tokens {
            0..=2_000 => "low".to_string(),
            2_001..=8_000 => "medium".to_string(),
            _ => "high".to_string(),
        });

        OpenAIRequest {
            model: request.model,
            messages,
            temperature: request.temperature,
            top_p: request.top_p,
            seed: request.seed,
            stop: request.stop,
            frequency_penalty: request.frequency_penalty,
            presence_penalty: request.presence_penalty,
            max_tokens: request.max_tokens,
            stream: Some(request.stream),
            stream_options: None, // set to include_usage=true by stream()
            tools,
            tool_choice,
            reasoning_effort,
            response_format: request.response_format,
        }
    }

    /// Convert OpenAI response to our generic format
    #[allow(clippy::wrong_self_convention)]
    fn from_openai_response(&self, response: OpenAIResponse) -> LLMResponse {
        let choice = response
            .choices
            .into_iter()
            .next()
            .unwrap_or_else(|| OpenAIChoice {
                index: 0,
                message: OpenAIMessage {
                    role: "assistant".to_string(),
                    content: Some(String::new()),
                    tool_calls: None,
                    tool_call_id: None,
                    reasoning_content: None,
                },
                finish_reason: Some("error".to_string()),
            });

        // Convert content to content blocks
        let mut content_blocks = Vec::new();

        // --- Reasoning / thinking block ---
        //
        // Priority:
        //  1. `reasoning_content` field (DeepSeek-R1 direct API, some OpenAI-compat servers)
        //  2. `<think>…</think>` tags embedded in the text content (Ollama DeepSeek-R1, QwQ-32B)
        let (thinking_text, visible_text) = {
            // Resolve reasoning_content first.
            let explicit_thinking = choice.message.reasoning_content.clone();

            match choice.message.content.as_deref() {
                Some(text) if !text.is_empty() => {
                    if let Some(r) = explicit_thinking.filter(|s| !s.is_empty()) {
                        // API provided reasoning separately — preserve content verbatim.
                        // Do NOT strip any tags, as they may be intentional display text.
                        (r, text.to_string())
                    } else {
                        // No separate reasoning field — extract from <think> tags if present.
                        let (tag_thinking, cleaned) =
                            crate::llm::provider::extract_think_tags(text);
                        (tag_thinking, cleaned)
                    }
                }
                _ => {
                    let thinking = explicit_thinking.unwrap_or_default();
                    (thinking, String::new())
                }
            }
        };

        if !thinking_text.is_empty() {
            content_blocks.push(ContentBlock::Thinking {
                thinking: thinking_text,
            });
        }

        // Add visible text content if present
        if !visible_text.is_empty() {
            content_blocks.push(ContentBlock::Text { text: visible_text });
        }

        // Convert tool_calls to ToolUse content blocks
        if let Some(tool_calls) = choice.message.tool_calls {
            tracing::debug!(
                "Converting {} tool calls from OpenAI response",
                tool_calls.len()
            );
            for tool_call in tool_calls {
                // Parse arguments JSON string
                let input =
                    serde_json::from_str(&tool_call.function.arguments).unwrap_or_else(|e| {
                        tracing::warn!(
                            "Failed to parse tool arguments for {}: {}",
                            tool_call.function.name,
                            e
                        );
                        serde_json::json!({})
                    });

                tracing::debug!(
                    "Converted tool call: {} with id {}",
                    tool_call.function.name,
                    tool_call.id
                );

                content_blocks.push(ContentBlock::ToolUse {
                    id: tool_call.id,
                    name: tool_call.function.name,
                    input,
                });
            }
        }

        // Map finish_reason to StopReason
        let stop_reason = choice
            .finish_reason
            .and_then(|reason| match reason.as_str() {
                "stop" => Some(StopReason::EndTurn),
                "length" => Some(StopReason::MaxTokens),
                "tool_calls" | "function_call" => Some(StopReason::ToolUse),
                _ => None,
            });

        LLMResponse {
            id: response.id,
            model: response.model,
            content: content_blocks,
            stop_reason,
            usage: TokenUsage {
                input_tokens: response
                    .usage
                    .as_ref()
                    .map(|u| u.prompt_tokens)
                    .unwrap_or(0),
                output_tokens: response
                    .usage
                    .as_ref()
                    .map(|u| u.completion_tokens)
                    .unwrap_or(0),
            },
            cache_metrics: None,
            perf_metrics: None,
        }
    }

    /// Handle API error response
    async fn handle_error(&self, response: reqwest::Response) -> ProviderError {
        let status = response.status().as_u16();

        // Extract Retry-After header for rate limits
        let retry_after = response.headers().get("retry-after").and_then(|v| {
            v.to_str().ok().and_then(|s| {
                // Retry-After can be either seconds or HTTP date
                // Try parsing as seconds first
                s.parse::<u64>().ok()
            })
        });

        // Try to parse error body
        if let Ok(error_body) = response.json::<OpenAIErrorResponse>().await {
            let message = if status == 429 {
                // Enhance rate limit error message
                if let Some(secs) = retry_after {
                    format!(
                        "{} (retry after {} seconds)",
                        error_body.error.message, secs
                    )
                } else {
                    format!(
                        "{} (rate limited, please retry later)",
                        error_body.error.message
                    )
                }
            } else {
                error_body.error.message
            };

            return if status == 429 {
                ProviderError::RateLimitExceeded(message)
            } else {
                ProviderError::ApiError {
                    status,
                    message,
                    error_type: Some(error_body.error.error_type.unwrap_or_default()),
                }
            };
        }

        // Fallback error
        if status == 429 {
            let message = if let Some(secs) = retry_after {
                format!("Rate limit exceeded (retry after {} seconds)", secs)
            } else {
                "Rate limit exceeded, please retry later".to_string()
            };
            ProviderError::RateLimitExceeded(message)
        } else {
            ProviderError::ApiError {
                status,
                message: "Unknown error".to_string(),
                error_type: None,
            }
        }
    }
}

#[async_trait]
impl Provider for OpenAIProvider {
    async fn complete(&self, request: LLMRequest) -> Result<LLMResponse> {
        use super::retry::{retry_with_backoff, RetryConfig};

        let model = request.model.clone();
        let message_count = request.messages.len();
        let openai_request = self.to_openai_request(request);
        let retry_config = RetryConfig::default();

        let tool_count = openai_request.tools.as_ref().map(|t| t.len()).unwrap_or(0);
        tracing::info!(
            "OpenAI API request: model={}, messages={}, max_tokens={}, tools={}",
            model,
            message_count,
            openai_request.max_tokens.unwrap_or(4096),
            tool_count
        );
        if tool_count == 0 {
            tracing::warn!(
                "OpenAI request has NO tools - LLM won't know about file/bash operations!"
            );
        }

        // Retry the entire API call with exponential backoff
        let result = retry_with_backoff(
            || async {
                tracing::debug!("Sending request to OpenAI API: {}", self.base_url);
                let response = self
                    .client
                    .post(&self.base_url)
                    .headers(self.headers()?)
                    .json(&openai_request)
                    .send()
                    .await?;

                let status = response.status();
                tracing::debug!("OpenAI API response status: {}", status);

                if !status.is_success() {
                    return Err(self.handle_error(response).await);
                }

                let openai_response: OpenAIResponse = response.json().await?;
                let llm_response = self.from_openai_response(openai_response);

                tracing::info!(
                    "OpenAI API response: input_tokens={}, output_tokens={}, stop_reason={:?}",
                    llm_response.usage.input_tokens,
                    llm_response.usage.output_tokens,
                    llm_response.stop_reason
                );

                Ok(llm_response)
            },
            &retry_config,
        )
        .await;

        if let Err(ref e) = result {
            tracing::error!("OpenAI API request failed: {}", e);
        }

        result
    }

    async fn stream(&self, request: LLMRequest) -> Result<ProviderStream> {
        use super::retry::{retry_with_backoff, RetryConfig};

        let model = request.model.clone();
        let message_count = request.messages.len();
        tracing::info!(
            "OpenAI streaming request: model={}, messages={}",
            model,
            message_count
        );

        let mut openai_request = self.to_openai_request(request);
        openai_request.stream = Some(true);
        // Request token usage in the final SSE chunk — required for Ollama to
        // report non-zero counts in streaming mode.
        openai_request.stream_options = Some(OpenAIStreamOptions {
            include_usage: true,
        });
        let retry_config = RetryConfig::default();

        // Retry the stream connection establishment
        let response = retry_with_backoff(
            || async {
                let response = self
                    .client
                    .post(&self.base_url)
                    .headers(self.headers()?)
                    .json(&openai_request)
                    .send()
                    .await?;

                if !response.status().is_success() {
                    return Err(self.handle_error(response).await);
                }

                Ok(response)
            },
            &retry_config,
        )
        .await?;

        // Parse the Server-Sent Events as the bytes arrive, so text reaches the
        // caller token by token. Dropping the returned stream drops the HTTP
        // response with it.
        use futures::StreamExt as _;
        let bytes = Box::pin(response.bytes_stream());
        let events = futures::stream::unfold(
            (bytes, Some(OpenAIStreamState::new(model))),
            |(mut bytes, state)| async move {
                let mut state = state?;
                let events = match bytes.next().await {
                    Some(Ok(chunk)) => {
                        let mut events = state.on_bytes(&chunk);
                        if !state.done {
                            return Some((ok_events(events), (bytes, Some(state))));
                        }
                        // `[DONE]` arrived: finish without waiting for the
                        // server to close the connection.
                        events.extend(state.finish());
                        ok_events(events)
                    }
                    Some(Err(e)) => vec![Err(ProviderError::StreamError(e.to_string()))],
                    None => ok_events(state.finish()),
                };
                Some((events, (bytes, None)))
            },
        )
        .flat_map(futures::stream::iter);

        Ok(Box::pin(events))
    }

    fn supports_streaming(&self) -> bool {
        true
    }

    fn supports_tools(&self) -> bool {
        true
    }

    fn supports_vision(&self) -> bool {
        // Detect common vision-capable model names served through Ollama and
        // other OpenAI-compatible backends. Shared with OllamaProvider so both
        // stay in sync (see `model_hints::is_vision_model`).
        super::model_hints::is_vision_model(self.custom_default_model.as_deref().unwrap_or(""))
    }

    fn name(&self) -> &str {
        "openai"
    }

    fn default_model(&self) -> &str {
        self.custom_default_model
            .as_deref()
            .unwrap_or("gpt-4-turbo-preview")
    }

    fn supported_models(&self) -> Vec<String> {
        vec![
            "gpt-4-turbo-preview".to_string(),
            "gpt-4".to_string(),
            "gpt-4-32k".to_string(),
            "gpt-3.5-turbo".to_string(),
            "gpt-3.5-turbo-16k".to_string(),
        ]
    }

    fn context_window(&self, model: &str) -> Option<u32> {
        match model {
            "gpt-4-turbo-preview" => Some(128_000),
            "gpt-4" => Some(8_192),
            "gpt-4-32k" => Some(32_768),
            "gpt-3.5-turbo" => Some(4_096),
            "gpt-3.5-turbo-16k" => Some(16_384),
            // Return a conservative default for unknown models (e.g. Ollama-hosted models
            // like "llama3.2", "qwen2.5-coder:7b", etc.).  Most current local models
            // support at least 8 K tokens; without this, service.rs falls back to 4 096
            // and aggressively truncates conversation context.
            _ => Some(8_192),
        }
    }

    fn calculate_cost(&self, model: &str, input_tokens: u32, output_tokens: u32) -> f64 {
        if let Some(cost) = llm_prices::Provider::OpenAi
            .lookup(model)
            .and_then(|p| p.cost(input_tokens as u64, output_tokens as u64))
        {
            return cost;
        }
        // Costs per million tokens (as of 2024)
        let (input_cost, output_cost) = match model {
            "gpt-4-turbo-preview" => (10.0, 30.0),
            "gpt-4" => (30.0, 60.0),
            "gpt-4-32k" => (60.0, 120.0),
            "gpt-3.5-turbo" => (0.5, 1.5),
            "gpt-3.5-turbo-16k" => (3.0, 4.0),
            _ => {
                tracing::warn!(
                    "Unknown OpenAI model '{}' for cost calculation - recording $0.00",
                    model
                );
                return 0.0;
            }
        };

        let input_cost_total = (input_tokens as f64 / 1_000_000.0) * input_cost;
        let output_cost_total = (output_tokens as f64 / 1_000_000.0) * output_cost;

        input_cost_total + output_cost_total
    }
}

// ============================================================================
// OpenAI API Types
// ============================================================================

#[derive(Debug, Clone, Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    /// Nucleus sampling (0.0–1.0). Use instead of temperature, not alongside.
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    /// Random seed for reproducible outputs (Ollama + OpenAI).
    #[serde(skip_serializing_if = "Option::is_none")]
    seed: Option<u64>,
    /// Stop sequences — generation halts at first match.
    #[serde(skip_serializing_if = "Option::is_none")]
    stop: Option<Vec<String>>,
    /// Penalises already-seen tokens (−2.0..2.0); reduces repetition.
    #[serde(skip_serializing_if = "Option::is_none")]
    frequency_penalty: Option<f32>,
    /// Penalises tokens that have appeared at all (−2.0..2.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
    /// When streaming, request token usage in the final SSE chunk.
    /// Required for Ollama — without this, streaming responses never include usage.
    #[serde(skip_serializing_if = "Option::is_none")]
    stream_options: Option<OpenAIStreamOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<OpenAITool>>,
    /// Set to "auto" whenever tools are present so local models (e.g. Ollama)
    /// reliably call tools instead of responding with plain text.
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<String>,
    /// Maps from ThinkingConfig to Ollama/OpenAI reasoning_effort levels.
    /// Enables reasoning traces for thinking models (DeepSeek-R1, Qwen3, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    reasoning_effort: Option<String>,
    /// JSON mode or structured output schema (OpenAI `response_format`).
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<serde_json::Value>,
}

/// Controls streaming behaviour extras sent alongside `"stream": true`.
#[derive(Debug, Clone, Serialize)]
struct OpenAIStreamOptions {
    /// When true, the final SSE chunk includes a `usage` object with token counts.
    include_usage: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OpenAIMessage {
    role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<OpenAIToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_call_id: Option<String>,
    /// Reasoning trace returned by some models (e.g. DeepSeek-R1 via its own API).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    reasoning_content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OpenAIToolCall {
    id: String,
    r#type: String,
    function: OpenAIFunctionCall,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OpenAIFunctionCall {
    name: String,
    arguments: String,
}

#[derive(Debug, Clone, Serialize)]
struct OpenAITool {
    r#type: String,
    function: OpenAIFunction,
}

#[derive(Debug, Clone, Serialize)]
struct OpenAIFunction {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize)]
struct OpenAIResponse {
    id: String,
    model: String,
    choices: Vec<OpenAIChoice>,
    // Ollama and some other local backends omit `usage` in certain responses.
    #[serde(default)]
    usage: Option<OpenAIUsage>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
struct OpenAIChoice {
    index: u32,
    message: OpenAIMessage,
    finish_reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct OpenAIUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
struct OpenAIStreamChunk {
    id: String,
    /// Some providers omit `model` from streaming chunks.
    #[serde(default)]
    model: Option<String>,
    choices: Vec<OpenAIStreamChoice>,
    /// Present in the final chunk when `stream_options.include_usage` is true.
    #[serde(default)]
    usage: Option<OpenAIUsage>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
struct OpenAIStreamChoice {
    index: u32,
    delta: Option<OpenAIMessageDelta>,
    finish_reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
struct OpenAIMessageDelta {
    role: Option<String>,
    content: Option<String>,
    // Tool call deltas arrive as partial fragments across multiple SSE chunks.
    // Each entry may only carry `index`, `id`, `type`, or `function.{name,arguments}`.
    #[serde(default)]
    tool_calls: Vec<OpenAIToolCallDelta>,
}

/// A single streamed fragment of a tool call.
#[derive(Debug, Clone, Deserialize, Default)]
#[allow(dead_code)]
struct OpenAIToolCallDelta {
    /// Position in the tool_calls array (used to merge fragments).
    index: usize,
    /// Present only in the first chunk for this tool call.
    #[serde(default)]
    id: Option<String>,
    #[serde(rename = "type", default)]
    call_type: Option<String>,
    #[serde(default)]
    function: Option<OpenAIFunctionDelta>,
}

/// Streamed function name/arguments fragment.
#[derive(Debug, Clone, Deserialize, Default)]
struct OpenAIFunctionDelta {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    arguments: Option<String>,
}

fn ok_events(events: Vec<StreamEvent>) -> Vec<Result<StreamEvent>> {
    events.into_iter().map(Ok).collect()
}

/// A tool call assembled from its streamed fragments.
#[derive(Default)]
struct StreamedToolCall {
    id: String,
    name: String,
    arguments: String,
}

/// Parses an OpenAI-compatible Server-Sent Events body into [`StreamEvent`]s
/// as the bytes arrive.
///
/// Text deltas are forwarded as soon as their line is complete. Tool calls
/// arrive as fragments spread over many chunks:
///
/// ```text
/// chunk 1: { tool_calls: [{ index:0, id:"call_abc", type:"function", function:{name:"read_file"} }] }
/// chunk 2: { tool_calls: [{ index:0, function:{arguments:"{\"path\":"} }] }
/// chunk 3: { tool_calls: [{ index:0, function:{arguments:"\"src/main.rs\"}"} }] }
/// chunk N: { finish_reason: "tool_calls" }
/// ```
///
/// so they are assembled here and emitted as `ToolUse` blocks when the stream ends.
struct OpenAIStreamState {
    /// Model name from the request, used when the chunks don't carry one.
    request_model: String,
    /// Bytes of an SSE line not yet terminated by `\n`.
    pending_line: Vec<u8>,
    message_started: bool,
    text_block_started: bool,
    tool_calls: Vec<StreamedToolCall>,
    finish_reason: Option<String>,
    /// From the final chunk when `stream_options.include_usage` is set.
    usage: Option<OpenAIUsage>,
    /// Set once the `[DONE]` marker has been read.
    done: bool,
}

impl OpenAIStreamState {
    fn new(request_model: String) -> Self {
        Self {
            request_model,
            pending_line: Vec::new(),
            message_started: false,
            text_block_started: false,
            tool_calls: Vec::new(),
            finish_reason: None,
            usage: None,
            done: false,
        }
    }

    /// Events for the complete SSE lines in `bytes`, available immediately.
    fn on_bytes(&mut self, bytes: &[u8]) -> Vec<StreamEvent> {
        let mut events = Vec::new();
        if self.done {
            return events;
        }
        self.pending_line.extend_from_slice(bytes);
        while let Some(end) = self.pending_line.iter().position(|&b| b == b'\n') {
            let line: Vec<u8> = self.pending_line.drain(..=end).collect();
            // A complete line never ends inside a multi-byte UTF-8 character,
            // so decoding line by line is safe even when a network chunk
            // splits one.
            let line = String::from_utf8_lossy(&line);
            self.on_line(line.trim_end_matches(['\n', '\r']), &mut events);
            if self.done {
                self.pending_line.clear();
                break;
            }
        }
        events
    }

    fn on_line(&mut self, line: &str, events: &mut Vec<StreamEvent>) {
        let Some(json_str) = line.strip_prefix("data: ") else {
            if !line.trim().is_empty()
                && !line.starts_with("event:")
                && !line.starts_with("id:")
                && !line.starts_with("retry:")
            {
                tracing::debug!("OpenAI: Unexpected SSE line format: {}", line);
            }
            return;
        };

        if json_str == "[DONE]" {
            tracing::trace!("OpenAI stream completed with [DONE] marker");
            self.done = true;
            return;
        }

        let chunk = match serde_json::from_str::<OpenAIStreamChunk>(json_str) {
            Ok(chunk) => chunk,
            Err(e) => {
                tracing::warn!(
                    "Failed to parse OpenAI stream chunk: {}. Data: {}",
                    e,
                    json_str.chars().take(200).collect::<String>()
                );
                return;
            }
        };

        // The first chunk carries the id and model for MessageStart.
        if !self.message_started {
            self.message_started = true;
            events.push(StreamEvent::MessageStart {
                message: StreamMessage {
                    id: chunk.id.clone(),
                    model: chunk
                        .model
                        .clone()
                        .unwrap_or_else(|| self.request_model.clone()),
                    role: Role::Assistant,
                    usage: TokenUsage {
                        input_tokens: 0,
                        output_tokens: 0,
                    },
                },
            });
        }
        if chunk.usage.is_some() {
            self.usage = chunk.usage;
        }

        let Some(choice) = chunk.choices.first() else {
            return;
        };
        if let Some(reason) = choice.finish_reason.as_ref().filter(|r| !r.is_empty()) {
            self.finish_reason = Some(reason.clone());
        }
        let Some(delta) = &choice.delta else {
            return;
        };

        if let Some(content) = delta.content.as_ref().filter(|c| !c.is_empty()) {
            if !self.text_block_started {
                self.text_block_started = true;
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
                    text: content.clone(),
                },
            });
        }

        for fragment in &delta.tool_calls {
            let idx = fragment.index;
            if !tool_call_index_in_bounds(idx) {
                tracing::warn!("Ignoring tool call delta with out-of-range index {}", idx);
                continue;
            }
            if idx >= self.tool_calls.len() {
                self.tool_calls.resize_with(idx + 1, Default::default);
            }
            let call = &mut self.tool_calls[idx];
            if let Some(id) = &fragment.id {
                call.id.clone_from(id);
            }
            if let Some(function) = &fragment.function {
                if let Some(name) = &function.name {
                    call.name.clone_from(name);
                }
                if let Some(arguments) = &function.arguments {
                    call.arguments.push_str(arguments);
                }
            }
        }
    }

    /// Closing events once the byte stream has ended or `[DONE]` was read.
    fn finish(mut self) -> Vec<StreamEvent> {
        let mut events = Vec::new();

        // A last line the server did not terminate with a newline.
        if !self.done && !self.pending_line.is_empty() {
            let line =
                String::from_utf8_lossy(&std::mem::take(&mut self.pending_line)).into_owned();
            self.on_line(line.trim_end_matches('\r'), &mut events);
        }

        // MessageStart must be the first event even if no chunk parsed.
        if !self.message_started {
            events.insert(
                0,
                StreamEvent::MessageStart {
                    message: StreamMessage {
                        id: "unknown".to_string(),
                        model: self.request_model.clone(),
                        role: Role::Assistant,
                        usage: TokenUsage {
                            input_tokens: 0,
                            output_tokens: 0,
                        },
                    },
                },
            );
        }

        // Close the text block before emitting any tool call blocks.
        if self.text_block_started {
            events.push(StreamEvent::ContentBlockStop { index: 0 });
        }

        if !self.tool_calls.is_empty() {
            tracing::debug!(
                "OpenAI stream: flushing {} tool call(s) accumulated from deltas",
                self.tool_calls.len()
            );
            // Block indices start at 1 when a text block was opened at index 0.
            let base_index = usize::from(self.text_block_started);
            for (i, call) in std::mem::take(&mut self.tool_calls).into_iter().enumerate() {
                // Skip incomplete tool calls from truncated or malformed responses.
                if call.id.is_empty() || call.name.is_empty() {
                    tracing::warn!(
                        "Skipping incomplete streamed tool call at index {}: id='{}', name='{}'",
                        i,
                        call.id,
                        call.name
                    );
                    continue;
                }
                let input = serde_json::from_str(&call.arguments).unwrap_or_else(|e| {
                    tracing::warn!(
                        "Failed to parse streamed tool arguments for '{}': {}",
                        call.name,
                        e
                    );
                    serde_json::json!({})
                });
                let block_index = base_index + i;
                events.push(StreamEvent::ContentBlockStart {
                    index: block_index,
                    content_block: ContentBlock::ToolUse {
                        id: call.id,
                        name: call.name,
                        input,
                    },
                });
                events.push(StreamEvent::ContentBlockStop { index: block_index });
            }
        }

        let stop_reason = self.finish_reason.as_deref().and_then(|r| match r {
            "tool_calls" | "function_call" => Some(StopReason::ToolUse),
            "length" => Some(StopReason::MaxTokens),
            "stop" => Some(StopReason::EndTurn),
            _ => None,
        });
        if stop_reason.is_some() {
            events.push(StreamEvent::MessageDelta {
                delta: MessageDelta {
                    stop_reason,
                    stop_sequence: None,
                },
                usage: TokenUsage {
                    input_tokens: self.usage.as_ref().map_or(0, |u| u.prompt_tokens),
                    output_tokens: self.usage.as_ref().map_or(0, |u| u.completion_tokens),
                },
                perf_metrics: None,
            });
        }
        events.push(StreamEvent::MessageStop);
        events
    }
}

#[derive(Debug, Clone, Deserialize)]
struct OpenAIErrorResponse {
    error: OpenAIError,
}

#[derive(Debug, Clone, Deserialize)]
struct OpenAIError {
    message: String,
    #[serde(rename = "type")]
    error_type: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openai_provider_creation() {
        let provider = OpenAIProvider::new("test-key".to_string());
        assert_eq!(provider.name(), "openai");
        assert_eq!(provider.base_url, DEFAULT_OPENAI_API_URL);
    }

    #[test]
    fn test_local_provider_creation() {
        let provider =
            OpenAIProvider::local("http://localhost:1234/v1/chat/completions".to_string());
        assert_eq!(provider.api_key, "not-needed");
    }

    /// Regression: `AzureOpenAIProvider` wraps `OpenAIProvider`, whose
    /// `headers()` unconditionally sent `Authorization: Bearer <key>`.
    /// Azure OpenAI's REST API requires the plain `api-key: <key>` header
    /// for key-based auth - `Bearer` is only valid there for Entra ID/AAD
    /// tokens - so every real Azure OpenAI request failed 401 Unauthorized
    /// regardless of how valid the configured key was.
    #[test]
    fn with_api_key_header_sends_api_key_not_bearer() {
        let provider = OpenAIProvider::with_base_url(
            "azure-secret".to_string(),
            "https://example.openai.azure.com/openai/deployments/x/chat/completions".to_string(),
        )
        .with_api_key_header();

        let headers = provider.headers().expect("headers build");
        assert_eq!(
            headers.get("api-key").and_then(|v| v.to_str().ok()),
            Some("azure-secret"),
            "Azure auth must use a plain api-key header"
        );
        assert!(
            !headers.contains_key(reqwest::header::AUTHORIZATION),
            "must not also send Authorization: Bearer - Azure key-based auth doesn't accept it"
        );
    }

    /// The default (non-Azure) auth style must be unaffected.
    #[test]
    fn default_auth_style_still_sends_bearer() {
        let provider = OpenAIProvider::new("plain-openai-key".to_string());
        let headers = provider.headers().expect("headers build");
        assert_eq!(
            headers
                .get(reqwest::header::AUTHORIZATION)
                .and_then(|v| v.to_str().ok()),
            Some("Bearer plain-openai-key")
        );
        assert!(!headers.contains_key("api-key"));
    }

    #[test]
    fn test_supported_models() {
        let provider = OpenAIProvider::new("test-key".to_string());
        let models = provider.supported_models();
        assert!(models.contains(&"gpt-4".to_string()));
        assert!(models.contains(&"gpt-3.5-turbo".to_string()));
    }

    #[test]
    fn test_context_window() {
        let provider = OpenAIProvider::new("test-key".to_string());
        assert_eq!(provider.context_window("gpt-4"), Some(8_192));
        assert_eq!(
            provider.context_window("gpt-4-turbo-preview"),
            Some(128_000)
        );
        // Unknown models (e.g. Ollama-hosted) get a conservative 8 K default
        // instead of None so service.rs does not hard-cap context at 4 096.
        assert_eq!(provider.context_window("unknown"), Some(8_192));
        assert_eq!(provider.context_window("llama3.2"), Some(8_192));
    }

    #[test]
    fn test_supports_vision_detection() {
        let provider_llava =
            OpenAIProvider::local("http://localhost:11434/v1/chat/completions".to_string())
                .with_default_model("llava:13b".to_string());
        assert!(provider_llava.supports_vision());

        let provider_vision =
            OpenAIProvider::local("http://localhost:11434/v1/chat/completions".to_string())
                .with_default_model("llama3.2-vision:11b".to_string());
        assert!(provider_vision.supports_vision());

        let provider_plain =
            OpenAIProvider::local("http://localhost:11434/v1/chat/completions".to_string())
                .with_default_model("llama3.2:8b".to_string());
        assert!(!provider_plain.supports_vision());

        let provider_gpt4 = OpenAIProvider::new("test-key".to_string());
        assert!(!provider_gpt4.supports_vision()); // no default model set
    }

    #[test]
    fn test_llm_request_new_fields() {
        use crate::llm::provider::types::{LLMRequest, Message};
        let req = LLMRequest::new("llama3.2", vec![Message::user("hi")])
            .with_top_p(0.9)
            .with_seed(42)
            .with_stop(vec!["</s>".to_string()])
            .with_frequency_penalty(0.5)
            .with_presence_penalty(0.3)
            .with_response_format(serde_json::json!({"type": "json_object"}));

        assert_eq!(req.top_p, Some(0.9));
        assert_eq!(req.seed, Some(42));
        assert_eq!(req.stop, Some(vec!["</s>".to_string()]));
        assert_eq!(req.frequency_penalty, Some(0.5));
        assert_eq!(req.presence_penalty, Some(0.3));
        assert!(req.response_format.is_some());
    }

    #[test]
    fn test_new_fields_forwarded_to_openai_request() {
        use crate::llm::provider::types::{LLMRequest, Message};
        let provider = OpenAIProvider::new("test-key".to_string());
        let req = LLMRequest::new("gpt-4", vec![Message::user("hi")])
            .with_top_p(0.8)
            .with_seed(99)
            .with_stop(vec!["STOP".to_string()])
            .with_frequency_penalty(0.2)
            .with_presence_penalty(0.1)
            .with_response_format(serde_json::json!({"type": "json_object"}));

        let openai_req = provider.to_openai_request(req);
        assert_eq!(openai_req.top_p, Some(0.8));
        assert_eq!(openai_req.seed, Some(99));
        assert_eq!(openai_req.stop, Some(vec!["STOP".to_string()]));
        assert_eq!(openai_req.frequency_penalty, Some(0.2));
        assert_eq!(openai_req.presence_penalty, Some(0.1));
        assert!(openai_req.response_format.is_some());
    }

    /// Notes about the files a tool used ride in the tool-result message as a
    /// text block; they must reach the model as a user message after the
    /// `tool` messages, not be dropped.
    #[test]
    fn test_text_sent_with_tool_results_follows_them() {
        use crate::llm::provider::types::{ContentBlock, LLMRequest, Message, Role};
        let provider = OpenAIProvider::new("test-key".to_string());
        let req = LLMRequest::new(
            "gpt-4",
            vec![Message {
                role: Role::User,
                content: vec![
                    ContentBlock::ToolResult {
                        tool_use_id: "call_1".to_string(),
                        content: "file contents".to_string(),
                        is_error: None,
                    },
                    ContentBlock::Text {
                        text: "<recalled_memories>note</recalled_memories>".to_string(),
                    },
                ],
            }],
        );

        let openai_req = provider.to_openai_request(req);
        let n = openai_req.messages.len();
        assert!(n >= 2);
        assert_eq!(openai_req.messages[n - 2].role, "tool");
        assert_eq!(openai_req.messages[n - 1].role, "user");
        assert!(openai_req.messages[n - 1]
            .content
            .as_deref()
            .unwrap_or("")
            .contains("note"));
    }

    #[test]
    fn test_calculate_cost() {
        let provider = OpenAIProvider::new("test-key".to_string());
        // 1000 input + 1000 output tokens on gpt-3.5-turbo
        // Cost: (1000/1M * 0.5) + (1000/1M * 1.5) = 0.0005 + 0.0015 = 0.002
        let cost = provider.calculate_cost("gpt-3.5-turbo", 1000, 1000);
        assert!((cost - 0.002).abs() < 0.0001);
    }

    #[test]
    fn test_calculate_cost_current_models_are_not_free() {
        let provider = OpenAIProvider::new("test-key".to_string());
        // gpt-4o: $2.50 input + $10.00 output per 1M tokens
        let cost = provider.calculate_cost("gpt-4o", 1_000_000, 1_000_000);
        assert!((cost - 12.5).abs() < 1e-9);
        for model in ["gpt-4o-mini", "gpt-4.1", "gpt-5", "o3"] {
            assert!(provider.calculate_cost(model, 1000, 1000) > 0.0, "{model}");
        }
    }

    #[test]
    fn test_calculate_cost_unknown_model_returns_zero() {
        let provider = OpenAIProvider::new("test-key".to_string());
        assert_eq!(
            provider.calculate_cost("gpt-5-nonexistent", 1000, 1000),
            0.0
        );
    }

    #[test]
    fn test_tool_call_index_in_bounds() {
        assert!(tool_call_index_in_bounds(0));
        assert!(tool_call_index_in_bounds(MAX_TOOL_CALL_INDEX));
        assert!(!tool_call_index_in_bounds(MAX_TOOL_CALL_INDEX + 1));
        assert!(!tool_call_index_in_bounds(500_000_000));
    }
}

#[cfg(test)]
mod streaming_tests {
    use super::*;
    use crate::llm::provider::test_support::held_response_server;
    use futures::StreamExt as _;
    use serde_json::json;
    use std::time::Duration;

    fn sse(value: serde_json::Value) -> String {
        format!("data: {value}\n\n")
    }

    fn text_chunk(text: &str, finish_reason: Option<&str>) -> serde_json::Value {
        json!({
            "id": "c1",
            "model": "m",
            "choices": [{ "index": 0, "delta": { "content": text }, "finish_reason": finish_reason }]
        })
    }

    fn texts(events: &[StreamEvent]) -> Vec<String> {
        events
            .iter()
            .filter_map(|e| match e {
                StreamEvent::ContentBlockDelta {
                    delta: ContentDelta::TextDelta { text },
                    ..
                } => Some(text.clone()),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn text_is_forwarded_once_its_line_is_complete() {
        let mut state = OpenAIStreamState::new("m".to_string());
        let line = sse(text_chunk("Café", None));
        // Split the bytes inside the two-byte "é".
        let split = line.find('é').expect("accent present") + 1;

        assert!(state.on_bytes(&line.as_bytes()[..split]).is_empty());
        let events = state.on_bytes(&line.as_bytes()[split..]);
        assert!(matches!(events[0], StreamEvent::MessageStart { .. }));
        assert_eq!(texts(&events), ["Café"]);

        let next = state.on_bytes(sse(text_chunk(" au lait", None)).as_bytes());
        assert!(
            !next
                .iter()
                .any(|e| matches!(e, StreamEvent::MessageStart { .. })),
            "MessageStart is sent once"
        );
        assert_eq!(texts(&next), [" au lait"]);
    }

    #[test]
    fn fragmented_tool_call_is_assembled_when_done() {
        let mut state = OpenAIStreamState::new("m".to_string());
        let body = [
            sse(json!({"id": "c1", "choices": [{"index": 0, "delta": {"tool_calls": [
                {"index": 0, "id": "call_1", "type": "function", "function": {"name": "read_file"}}
            ]}, "finish_reason": null}]})),
            sse(json!({"id": "c1", "choices": [{"index": 0, "delta": {"tool_calls": [
                {"index": 0, "function": {"arguments": "{\"path\":"}}
            ]}, "finish_reason": null}]})),
            sse(json!({"id": "c1", "choices": [{"index": 0, "delta": {"tool_calls": [
                {"index": 0, "function": {"arguments": "\"a.rs\"}"}}
            ]}, "finish_reason": null}]})),
            sse(json!({"id": "c1", "choices": [{"index": 0, "delta": {}, "finish_reason": "tool_calls"}],
                       "usage": {"prompt_tokens": 7, "completion_tokens": 5}})),
            "data: [DONE]\n\n".to_string(),
        ]
        .concat();

        let mut events = state.on_bytes(body.as_bytes());
        assert!(state.done, "[DONE] read");
        assert!(
            state.on_bytes(b"data: ignored\n\n").is_empty(),
            "nothing after [DONE]"
        );
        events.extend(state.finish());

        let (name, input) = events
            .iter()
            .find_map(|e| match e {
                StreamEvent::ContentBlockStart {
                    content_block: ContentBlock::ToolUse { name, input, .. },
                    ..
                } => Some((name.clone(), input.clone())),
                _ => None,
            })
            .expect("tool call assembled");
        assert_eq!(name, "read_file");
        assert_eq!(input["path"], "a.rs");
        let (stop, usage) = events
            .iter()
            .find_map(|e| match e {
                StreamEvent::MessageDelta { delta, usage, .. } => {
                    Some((delta.stop_reason.clone(), *usage))
                }
                _ => None,
            })
            .expect("stop reason sent");
        assert_eq!(stop, Some(StopReason::ToolUse));
        assert_eq!((usage.input_tokens, usage.output_tokens), (7, 5));
        assert!(matches!(events.last(), Some(StreamEvent::MessageStop)));
    }

    #[test]
    fn last_line_without_newline_is_parsed_at_the_end() {
        let mut state = OpenAIStreamState::new("m".to_string());
        let line = format!("data: {}", text_chunk("Bye", Some("stop")));

        assert!(state.on_bytes(line.as_bytes()).is_empty());
        let events = state.finish();
        assert!(matches!(events[0], StreamEvent::MessageStart { .. }));
        assert_eq!(texts(&events), ["Bye"]);
        assert!(matches!(events.last(), Some(StreamEvent::MessageStop)));
    }

    #[test]
    fn empty_stream_still_starts_and_stops_the_message() {
        let events = OpenAIStreamState::new("m".to_string()).finish();
        assert!(
            matches!(&events[0], StreamEvent::MessageStart { message } if message.model == "m")
        );
        assert!(matches!(events.last(), Some(StreamEvent::MessageStop)));
    }

    /// Regression: `stream()` used to read the whole response body before
    /// yielding any event, so callers saw nothing until the model had finished.
    #[tokio::test]
    async fn text_arrives_before_the_response_is_complete() {
        let (release, held) = tokio::sync::oneshot::channel();
        let url = held_response_server(
            "text/event-stream",
            sse(text_chunk("Hello", None)),
            sse(text_chunk(" there.", Some("stop"))) + "data: [DONE]\n\n",
            held,
        )
        .await;
        let provider = OpenAIProvider::local(format!("{url}/v1/chat/completions"));
        let request = LLMRequest::new("m", vec![Message::user("hi")]);

        // The server holds back the rest of the response: the first text must
        // already be here. A provider that buffers the body hangs instead.
        let mut stream = tokio::time::timeout(Duration::from_secs(10), async {
            let mut stream = provider.stream(request).await.expect("stream starts");
            loop {
                match stream.next().await {
                    Some(Ok(StreamEvent::ContentBlockDelta {
                        delta: ContentDelta::TextDelta { text },
                        ..
                    })) => {
                        assert_eq!(text, "Hello");
                        break stream;
                    }
                    Some(Ok(_)) => {}
                    other => panic!("stream ended before any text: {other:?}"),
                }
            }
        })
        .await
        .expect("first text arrives while the server is still holding the response");

        release.send(()).expect("server is waiting");
        let mut rest = Vec::new();
        while let Some(event) = stream.next().await {
            rest.push(event.expect("no stream error"));
        }
        assert_eq!(texts(&rest), [" there."]);
        assert!(matches!(rest.last(), Some(StreamEvent::MessageStop)));
    }
}
