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

/// OpenAI provider for GPT models
#[derive(Clone)]
pub struct OpenAIProvider {
    api_key: String,
    base_url: String,
    client: Client,
    custom_default_model: Option<String>,
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
        }
    }

    /// Set custom default model (useful for local LLMs with specific model names)
    pub fn with_default_model(mut self, model: String) -> Self {
        self.custom_default_model = Some(model);
        self
    }

    /// Build request headers
    fn headers(&self) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();

        // Only add authorization if not using local
        if self.api_key != "not-needed" {
            headers.insert(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", self.api_key)
                    .parse()
                    .expect("Invalid API key format"),
            );
        }

        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );

        headers
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
        let reasoning_effort = request.thinking.as_ref().map(|t| {
            match t.budget_tokens {
                0..=2_000 => "low".to_string(),
                2_001..=8_000 => "medium".to_string(),
                _ => "high".to_string(),
            }
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
            content_blocks.push(ContentBlock::Thinking { thinking: thinking_text });
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
                input_tokens: response.usage.as_ref().map(|u| u.prompt_tokens).unwrap_or(0),
                output_tokens: response.usage.as_ref().map(|u| u.completion_tokens).unwrap_or(0),
            },
            cache_metrics: None,
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
                    .headers(self.headers())
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
        openai_request.stream_options = Some(OpenAIStreamOptions { include_usage: true });
        let retry_config = RetryConfig::default();

        // Retry the stream connection establishment
        let response = retry_with_backoff(
            || async {
                let response = self
                    .client
                    .post(&self.base_url)
                    .headers(self.headers())
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

        // Parse Server-Sent Events stream.
        //
        // Tool calls arrive as *fragmented* deltas across many chunks:
        //   chunk 1: { tool_calls: [{ index:0, id:"call_abc", type:"function", function:{name:"read_file"} }] }
        //   chunk 2: { tool_calls: [{ index:0, function:{arguments:"{\"path\":"} }] }
        //   chunk 3: { tool_calls: [{ index:0, function:{arguments:"\"src/main.rs\"}"} }] }
        //   chunk N: { finish_reason: "tool_calls" }
        //
        // We accumulate fragments in `tool_call_builders` (indexed by tool call
        // position) and emit all ToolUse events just before the MessageStop.
        let byte_stream = response.bytes_stream();

        // Intermediate accumulator for a single tool call's fragments.
        #[derive(Default)]
        struct ToolCallBuilder {
            id: String,
            name: String,
            arguments: String,
        }

        // Collect the full byte stream first so we can hold mutable state while
        // processing.  Tool call JSON is fragmented across many SSE chunks and must
        // be fully assembled before it can be parsed.
        let mut tool_call_builders: Vec<ToolCallBuilder> = Vec::new();
        let mut events: Vec<StreamEvent> = Vec::new();
        let mut raw_bytes = Vec::<u8>::new();

        {
            use futures::StreamExt as _;
            let mut bs = byte_stream;
            while let Some(chunk_result) = bs.next().await {
                match chunk_result {
                    Ok(chunk) => raw_bytes.extend_from_slice(&chunk),
                    Err(e) => {
                        return Err(ProviderError::StreamError(e.to_string()));
                    }
                }
            }
        }

        let text = String::from_utf8_lossy(&raw_bytes);
        let mut finish_reason: Option<String> = None;
        // Captured from the first SSE chunk for MessageStart.
        let mut message_id: Option<String> = None;
        let mut stream_model: Option<String> = None;
        // Captured from the final SSE chunk when stream_options.include_usage is true.
        let mut stream_usage: Option<OpenAIUsage> = None;
        // Whether a text ContentBlock (index 0) has been opened via ContentBlockStart.
        let mut text_block_started = false;

        for line in text.lines() {
            let Some(json_str) = line.strip_prefix("data: ") else {
                if !line.trim().is_empty()
                    && !line.starts_with("event:")
                    && !line.starts_with("id:")
                    && !line.starts_with("retry:")
                {
                    tracing::debug!("OpenAI: Unexpected SSE line format: {}", line);
                }
                continue;
            };

            if json_str == "[DONE]" {
                tracing::trace!("OpenAI stream completed with [DONE] marker");
                break;
            }

            match serde_json::from_str::<OpenAIStreamChunk>(json_str) {
                Ok(chunk) => {
                    // Capture id and model from the first chunk for MessageStart.
                    if message_id.is_none() {
                        message_id = Some(chunk.id.clone());
                        stream_model = chunk.model.clone();
                    }
                    // Capture usage from the final chunk (stream_options.include_usage).
                    if chunk.usage.is_some() {
                        stream_usage = chunk.usage;
                    }

                    if let Some(choice) = chunk.choices.first() {
                        // Capture finish_reason for the final flush decision.
                        if let Some(ref reason) = choice.finish_reason {
                            if !reason.is_empty() {
                                finish_reason = Some(reason.clone());
                            }
                        }

                        if let Some(ref delta) = choice.delta {
                            // --- text content ---
                            if let Some(ref content) = delta.content {
                                if !content.is_empty() {
                                    // Open the text block on the first non-empty delta.
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
                                            text: content.clone(),
                                        },
                                    });
                                }
                            }

                            // --- tool call fragments ---
                            for tc_delta in &delta.tool_calls {
                                let idx = tc_delta.index;

                                // Grow the builder vec as needed.
                                if idx >= tool_call_builders.len() {
                                    tool_call_builders.resize_with(idx + 1, Default::default);
                                }
                                let builder = &mut tool_call_builders[idx];

                                if let Some(ref id) = tc_delta.id {
                                    builder.id.clone_from(id);
                                }
                                if let Some(ref func) = tc_delta.function {
                                    if let Some(ref name) = func.name {
                                        builder.name.clone_from(name);
                                    }
                                    if let Some(ref args) = func.arguments {
                                        builder.arguments.push_str(args);
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    tracing::warn!(
                        "Failed to parse OpenAI stream chunk: {}. Data: {}",
                        e,
                        json_str.chars().take(200).collect::<String>()
                    );
                }
            }
        }

        // Prepend MessageStart as the required first event in the stream.
        // Use id/model captured from the first SSE chunk; fall back to request values.
        events.insert(
            0,
            StreamEvent::MessageStart {
                message: StreamMessage {
                    id: message_id.unwrap_or_else(|| "unknown".to_string()),
                    model: stream_model.unwrap_or_else(|| model.clone()),
                    role: Role::Assistant,
                    usage: TokenUsage {
                        input_tokens: 0,
                        output_tokens: 0,
                    },
                },
            },
        );

        // Close the text block before emitting any tool call blocks.
        if text_block_started {
            events.push(StreamEvent::ContentBlockStop { index: 0 });
        }

        // Flush accumulated tool calls as ToolUse events before MessageStop.
        if !tool_call_builders.is_empty() {
            tracing::debug!(
                "OpenAI stream: flushing {} tool call(s) accumulated from deltas",
                tool_call_builders.len()
            );
            // Block indices start at 1 when a text block was opened at index 0,
            // or at 0 when the response contains only tool calls.
            let base_index: usize = if text_block_started { 1 } else { 0 };
            for (i, builder) in tool_call_builders.into_iter().enumerate() {
                // Skip incomplete tool calls produced by truncated/malformed responses.
                if builder.id.is_empty() || builder.name.is_empty() {
                    tracing::warn!(
                        "Skipping incomplete streamed tool call at index {}: id='{}', name='{}'",
                        i,
                        builder.id,
                        builder.name
                    );
                    continue;
                }
                let input = serde_json::from_str(&builder.arguments).unwrap_or_else(|e| {
                    tracing::warn!(
                        "Failed to parse streamed tool arguments for '{}': {}",
                        builder.name,
                        e
                    );
                    serde_json::json!({})
                });
                let block_index = base_index + i;
                events.push(StreamEvent::ContentBlockStart {
                    index: block_index,
                    content_block: ContentBlock::ToolUse {
                        id: builder.id,
                        name: builder.name,
                        input,
                    },
                });
                events.push(StreamEvent::ContentBlockStop { index: block_index });
            }
        }

        // Emit stop reason via MessageDelta then the terminal MessageStop.
        let stop_reason = finish_reason.as_deref().and_then(|r| match r {
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
                    input_tokens: stream_usage
                        .as_ref()
                        .map(|u| u.prompt_tokens)
                        .unwrap_or(0),
                    output_tokens: stream_usage
                        .as_ref()
                        .map(|u| u.completion_tokens)
                        .unwrap_or(0),
                },
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
        // Detect common vision-capable model names served through Ollama and
        // other OpenAI-compatible backends.  The check is intentionally broad
        // (substring match, case-insensitive) to handle tag variants like
        // "llava:13b", "llama3.2-vision:11b-instruct-fp16", etc.
        let model_lc = self
            .custom_default_model
            .as_deref()
            .unwrap_or("")
            .to_lowercase();
        let vision_patterns = [
            "llava",
            "vision",
            "minicpm-v",
            "bakllava",
            "moondream",
            "cogvlm",
            "qwen-vl",
            "qwenvl",
            "internvl",
            "phi-3-vision",
            "phi3-vision",
            "idefics",
        ];
        vision_patterns.iter().any(|p| model_lc.contains(p))
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
        // Costs per million tokens (as of 2024)
        let (input_cost, output_cost) = match model {
            "gpt-4-turbo-preview" => (10.0, 30.0),
            "gpt-4" => (30.0, 60.0),
            "gpt-4-32k" => (60.0, 120.0),
            "gpt-3.5-turbo" => (0.5, 1.5),
            "gpt-3.5-turbo-16k" => (3.0, 4.0),
            _ => return 0.0,
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

    #[test]
    fn test_calculate_cost() {
        let provider = OpenAIProvider::new("test-key".to_string());
        // 1000 input + 1000 output tokens on gpt-3.5-turbo
        // Cost: (1000/1M * 0.5) + (1000/1M * 1.5) = 0.0005 + 0.0015 = 0.002
        let cost = provider.calculate_cost("gpt-3.5-turbo", 1000, 1000);
        assert!((cost - 0.002).abs() < 0.0001);
    }
}
