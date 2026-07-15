//! Google Gemini Provider Implementation
//!
//! Implements the Provider trait for Google's Gemini API (Generative Language
//! API), which serves both proprietary Gemini models and Google's open-weight
//! Gemma models (Gemma 3, Gemma 4) through the same `generateContent` /
//! `streamGenerateContent` REST surface.
//!
//! ## Supported Models
//! - gemini-3-pro, gemini-2.5-pro, gemini-2.5-flash, gemini-2.5-flash-lite
//! - gemini-2.0-flash, gemini-2.0-flash-lite
//! - gemma-4-31b-it, gemma-4-26b-a4b-it (MoE)
//! - gemma-3-27b-it, gemma-3-12b-it, gemma-3-4b-it, gemma-3-1b-it
//!
//! ## API notes
//! - Auth via `x-goog-api-key` header (avoids leaking the key into URLs/logs).
//! - Roles are `user` / `model` (Gemini has no `assistant` or `system` role in
//!   `contents`; system prompts go in the top-level `systemInstruction` field).
//! - Tool calls round-trip by function *name*, not by call ID like OpenAI/
//!   Anthropic — `functionResponse.name` must match the `functionCall.name`
//!   that produced it, so the provider tracks a call-id → name map per request.

use super::error::{ProviderError, Result};
use super::r#trait::{Provider, ProviderStream};
use super::types::*;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

const DEFAULT_GEMINI_API_URL: &str = "https://generativelanguage.googleapis.com/v1beta";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(120);
const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
const DEFAULT_POOL_IDLE_TIMEOUT: Duration = Duration::from_secs(90);

/// Google Gemini provider — also serves Gemma models via the same API.
#[derive(Clone)]
pub struct GeminiProvider {
    api_key: String,
    base_url: String,
    client: Client,
    custom_default_model: Option<String>,
}

impl GeminiProvider {
    /// Create a new Gemini provider using the official Google AI API.
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
            base_url: DEFAULT_GEMINI_API_URL.to_string(),
            client,
            custom_default_model: None,
        }
    }

    /// Create with a custom base URL (e.g. a Vertex AI or proxy endpoint that
    /// speaks the same `generateContent` REST shape).
    pub fn with_base_url(api_key: String, base_url: String) -> Self {
        let mut provider = Self::new(api_key);
        provider.base_url = base_url;
        provider
    }

    /// Set a custom default model (e.g. pin to a specific Gemma variant).
    pub fn with_default_model(mut self, model: String) -> Self {
        self.custom_default_model = Some(model);
        self
    }

    fn headers(&self) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "x-goog-api-key",
            self.api_key.parse().expect("Invalid API key format"),
        );
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );
        headers
    }

    fn generate_url(&self, model: &str) -> String {
        format!("{}/models/{}:generateContent", self.base_url, model)
    }

    fn stream_url(&self, model: &str) -> String {
        format!(
            "{}/models/{}:streamGenerateContent?alt=sse",
            self.base_url, model
        )
    }

    /// Convert our generic request to Gemini's `generateContent` request shape.
    fn to_gemini_request(&self, request: &LLMRequest) -> GeminiRequest {
        let mut contents = Vec::new();
        // Function call id -> name, so a later ToolResult (keyed by id) can be
        // turned into a Gemini `functionResponse` (keyed by name).
        let mut call_names: HashMap<String, String> = HashMap::new();

        for msg in &request.messages {
            let mut text_parts = Vec::new();
            let mut function_calls = Vec::new();
            let mut function_responses = Vec::new();

            for block in &msg.content {
                match block {
                    ContentBlock::Text { text } => text_parts.push(text.clone()),
                    ContentBlock::ToolUse { id, name, input } => {
                        call_names.insert(id.clone(), name.clone());
                        function_calls.push(GeminiPart {
                            function_call: Some(GeminiFunctionCall {
                                name: name.clone(),
                                args: input.clone(),
                            }),
                            ..Default::default()
                        });
                    }
                    ContentBlock::ToolResult {
                        tool_use_id,
                        content,
                        ..
                    } => {
                        let name = call_names
                            .get(tool_use_id)
                            .cloned()
                            .unwrap_or_else(|| tool_use_id.clone());
                        function_responses.push(GeminiPart {
                            function_response: Some(GeminiFunctionResponse {
                                name,
                                response: serde_json::json!({ "result": content }),
                            }),
                            ..Default::default()
                        });
                    }
                    ContentBlock::Image { source } => match source {
                        ImageSource::Base64 { media_type, data } => {
                            contents.push(GeminiContent {
                                role: Some(gemini_role(&msg.role).to_string()),
                                parts: vec![GeminiPart::default()
                                    .with_inline_data(media_type.clone(), data.clone())],
                            });
                        }
                        ImageSource::Url { .. } => {
                            tracing::warn!(
                                "Gemini requires inline base64 image data; URL images are not supported"
                            );
                        }
                    },
                    ContentBlock::Thinking { .. } => {
                        // Thought signatures are not round-tripped; Gemini does not
                        // require replaying prior thinking content back to the model.
                    }
                }
            }

            if !function_calls.is_empty() {
                let mut parts = Vec::new();
                if !text_parts.is_empty() {
                    parts.push(GeminiPart::text(text_parts.join("\n")));
                }
                parts.extend(function_calls);
                contents.push(GeminiContent {
                    role: Some("model".to_string()),
                    parts,
                });
            } else if !function_responses.is_empty() {
                contents.push(GeminiContent {
                    role: Some("user".to_string()),
                    parts: function_responses,
                });
            } else if !text_parts.is_empty() {
                contents.push(GeminiContent {
                    role: Some(gemini_role(&msg.role).to_string()),
                    parts: vec![GeminiPart::text(text_parts.join("\n"))],
                });
            }
        }

        let system_instruction = request.system.as_ref().map(|s| GeminiContent {
            role: None,
            parts: vec![GeminiPart::text(s.clone())],
        });

        let tools = request.tools.as_ref().map(|tools| {
            vec![GeminiTool {
                function_declarations: tools
                    .iter()
                    .map(|t| GeminiFunctionDeclaration {
                        name: t.name.clone(),
                        description: t.description.clone(),
                        parameters: t.input_schema.clone(),
                    })
                    .collect(),
            }]
        });

        let tool_config = tools.as_ref().map(|_| GeminiToolConfig {
            function_calling_config: GeminiFunctionCallingConfig {
                mode: "AUTO".to_string(),
            },
        });

        // Gemini's thinkingBudget uses -1 for "dynamic"/model-chosen and 0 to
        // disable; our generic budget_tokens maps directly onto an explicit token cap.
        let thinking_config = request.thinking.as_ref().map(|t| GeminiThinkingConfig {
            thinking_budget: t.budget_tokens as i32,
            include_thoughts: Some(true),
        });

        let generation_config = GeminiGenerationConfig {
            temperature: request.temperature,
            top_p: request.top_p,
            max_output_tokens: request.max_tokens,
            stop_sequences: request.stop.clone(),
            response_mime_type: request
                .response_format
                .as_ref()
                .and_then(|f| f.get("type"))
                .and_then(|t| t.as_str())
                .filter(|t| *t == "json_object")
                .map(|_| "application/json".to_string()),
            response_schema: request.response_format.as_ref().and_then(|f| {
                // A full JSON Schema (not the `{"type":"json_object"}` marker) is
                // forwarded as Gemini's `responseSchema` for structured output.
                if f.get("type").and_then(|t| t.as_str()) == Some("json_object") {
                    None
                } else {
                    Some(f.clone())
                }
            }),
            thinking_config,
            seed: request.seed,
        };

        GeminiRequest {
            contents,
            system_instruction,
            tools,
            tool_config,
            generation_config: Some(generation_config),
        }
    }

    /// Convert a Gemini response into our generic format.
    #[allow(clippy::wrong_self_convention)]
    fn from_gemini_response(&self, response: GeminiResponse, model: &str) -> LLMResponse {
        let candidate = response.candidates.into_iter().next();

        let mut content_blocks = Vec::new();
        let mut finish_reason = None;

        if let Some(candidate) = candidate {
            finish_reason = candidate.finish_reason;
            if let Some(content) = candidate.content {
                for part in content.parts {
                    if let Some(text) = part.text {
                        if text.is_empty() {
                            continue;
                        }
                        if part.thought == Some(true) {
                            content_blocks.push(ContentBlock::Thinking { thinking: text });
                        } else {
                            content_blocks.push(ContentBlock::Text { text });
                        }
                    } else if let Some(fc) = part.function_call {
                        content_blocks.push(ContentBlock::ToolUse {
                            id: format!("gemini_call_{}", uuid::Uuid::new_v4()),
                            name: fc.name,
                            input: fc.args,
                        });
                    }
                }
            }
        }

        let stop_reason = finish_reason.as_deref().and_then(|r| match r {
            "STOP" => Some(StopReason::EndTurn),
            "MAX_TOKENS" => Some(StopReason::MaxTokens),
            _ if content_blocks
                .iter()
                .any(|b| matches!(b, ContentBlock::ToolUse { .. })) =>
            {
                Some(StopReason::ToolUse)
            }
            _ => None,
        });
        // A candidate carrying a function call always means the model wants to
        // invoke a tool, regardless of the raw finishReason Gemini reports.
        let stop_reason = if content_blocks
            .iter()
            .any(|b| matches!(b, ContentBlock::ToolUse { .. }))
        {
            Some(StopReason::ToolUse)
        } else {
            stop_reason
        };

        let usage = response
            .usage_metadata
            .map(|u| TokenUsage {
                input_tokens: u.prompt_token_count,
                output_tokens: u.candidates_token_count + u.thoughts_token_count,
            })
            .unwrap_or(TokenUsage {
                input_tokens: 0,
                output_tokens: 0,
            });

        LLMResponse {
            id: format!("gemini_{}", uuid::Uuid::new_v4()),
            model: model.to_string(),
            content: content_blocks,
            stop_reason,
            usage,
            cache_metrics: None,
            perf_metrics: None,
        }
    }

    async fn handle_error(&self, response: reqwest::Response) -> ProviderError {
        let status = response.status().as_u16();

        let retry_after = response
            .headers()
            .get("retry-after")
            .and_then(|v| v.to_str().ok().and_then(|s| s.parse::<u64>().ok()));

        if let Ok(error_body) = response.json::<GeminiErrorResponse>().await {
            let message = if status == 429 {
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
                    error_type: error_body.error.status,
                }
            };
        }

        if status == 429 {
            ProviderError::RateLimitExceeded("Rate limit exceeded, please retry later".to_string())
        } else {
            ProviderError::ApiError {
                status,
                message: "Unknown error".to_string(),
                error_type: None,
            }
        }
    }
}

fn gemini_role(role: &Role) -> &'static str {
    match role {
        Role::User | Role::System => "user",
        Role::Assistant => "model",
    }
}

#[async_trait]
impl Provider for GeminiProvider {
    async fn complete(&self, request: LLMRequest) -> Result<LLMResponse> {
        use super::retry::{retry_with_backoff, RetryConfig};

        let model = request.model.clone();
        let gemini_request = self.to_gemini_request(&request);
        let url = self.generate_url(&model);
        let retry_config = RetryConfig::default();

        tracing::info!(
            "Gemini API request: model={}, messages={}, tools={}",
            model,
            gemini_request.contents.len(),
            gemini_request
                .tools
                .as_ref()
                .map(|t| t
                    .iter()
                    .map(|d| d.function_declarations.len())
                    .sum::<usize>())
                .unwrap_or(0)
        );

        let result = retry_with_backoff(
            || async {
                let response = self
                    .client
                    .post(&url)
                    .headers(self.headers())
                    .json(&gemini_request)
                    .send()
                    .await?;

                let status = response.status();
                if !status.is_success() {
                    return Err(self.handle_error(response).await);
                }

                let gemini_response: GeminiResponse = response.json().await?;
                Ok(self.from_gemini_response(gemini_response, &model))
            },
            &retry_config,
        )
        .await;

        if let Err(ref e) = result {
            tracing::error!("Gemini API request failed: {}", e);
        }

        result
    }

    async fn stream(&self, request: LLMRequest) -> Result<ProviderStream> {
        use super::retry::{retry_with_backoff, RetryConfig};

        let model = request.model.clone();
        let gemini_request = self.to_gemini_request(&request);
        let url = self.stream_url(&model);
        let retry_config = RetryConfig::default();

        tracing::info!("Gemini streaming request: model={}", model);

        let response = retry_with_backoff(
            || async {
                let response = self
                    .client
                    .post(&url)
                    .headers(self.headers())
                    .json(&gemini_request)
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

        let byte_stream = response.bytes_stream();
        let mut raw_bytes = Vec::<u8>::new();
        {
            use futures::StreamExt as _;
            let mut bs = byte_stream;
            while let Some(chunk_result) = bs.next().await {
                match chunk_result {
                    Ok(chunk) => raw_bytes.extend_from_slice(&chunk),
                    Err(e) => return Err(ProviderError::StreamError(e.to_string())),
                }
            }
        }

        let text = String::from_utf8_lossy(&raw_bytes);
        let mut events: Vec<StreamEvent> = Vec::new();
        let mut text_block_started = false;
        let mut next_block_index = 0usize;
        let mut final_usage = TokenUsage {
            input_tokens: 0,
            output_tokens: 0,
        };
        let mut final_finish_reason: Option<String> = None;
        let mut saw_tool_call = false;

        events.push(StreamEvent::MessageStart {
            message: StreamMessage {
                id: format!("gemini_{}", uuid::Uuid::new_v4()),
                model: model.clone(),
                role: Role::Assistant,
                usage: TokenUsage {
                    input_tokens: 0,
                    output_tokens: 0,
                },
            },
        });

        for line in text.lines() {
            let Some(json_str) = line.strip_prefix("data: ") else {
                continue;
            };
            if json_str.trim().is_empty() {
                continue;
            }

            match serde_json::from_str::<GeminiResponse>(json_str) {
                Ok(chunk) => {
                    if let Some(usage) = chunk.usage_metadata {
                        final_usage = TokenUsage {
                            input_tokens: usage.prompt_token_count,
                            output_tokens: usage.candidates_token_count
                                + usage.thoughts_token_count,
                        };
                    }

                    if let Some(candidate) = chunk.candidates.into_iter().next() {
                        if let Some(reason) = candidate.finish_reason {
                            final_finish_reason = Some(reason);
                        }
                        if let Some(content) = candidate.content {
                            for part in content.parts {
                                if let Some(text) = part.text {
                                    if text.is_empty() {
                                        continue;
                                    }
                                    if part.thought == Some(true) {
                                        events.push(StreamEvent::ContentBlockDelta {
                                            index: 0,
                                            delta: ContentDelta::ThinkingDelta { thinking: text },
                                        });
                                        continue;
                                    }
                                    if !text_block_started {
                                        text_block_started = true;
                                        events.push(StreamEvent::ContentBlockStart {
                                            index: 0,
                                            content_block: ContentBlock::Text {
                                                text: String::new(),
                                            },
                                        });
                                        next_block_index = 1;
                                    }
                                    events.push(StreamEvent::ContentBlockDelta {
                                        index: 0,
                                        delta: ContentDelta::TextDelta { text },
                                    });
                                } else if let Some(fc) = part.function_call {
                                    saw_tool_call = true;
                                    let idx = next_block_index;
                                    next_block_index += 1;
                                    events.push(StreamEvent::ContentBlockStart {
                                        index: idx,
                                        content_block: ContentBlock::ToolUse {
                                            id: format!("gemini_call_{}", uuid::Uuid::new_v4()),
                                            name: fc.name,
                                            input: fc.args,
                                        },
                                    });
                                    events.push(StreamEvent::ContentBlockStop { index: idx });
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    tracing::warn!(
                        "Failed to parse Gemini stream chunk: {}. Data: {}",
                        e,
                        json_str.chars().take(200).collect::<String>()
                    );
                }
            }
        }

        if text_block_started {
            events.push(StreamEvent::ContentBlockStop { index: 0 });
        }

        let stop_reason = if saw_tool_call {
            Some(StopReason::ToolUse)
        } else {
            final_finish_reason.as_deref().and_then(|r| match r {
                "STOP" => Some(StopReason::EndTurn),
                "MAX_TOKENS" => Some(StopReason::MaxTokens),
                _ => None,
            })
        };

        if stop_reason.is_some() {
            events.push(StreamEvent::MessageDelta {
                delta: MessageDelta {
                    stop_reason,
                    stop_sequence: None,
                },
                usage: final_usage,
                perf_metrics: None,
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
        // Every currently-served Gemini and Gemma model accepts inline image parts.
        true
    }

    fn name(&self) -> &str {
        "gemini"
    }

    fn default_model(&self) -> &str {
        self.custom_default_model
            .as_deref()
            .unwrap_or("gemini-2.5-flash")
    }

    fn supported_models(&self) -> Vec<String> {
        vec![
            "gemini-3-pro".to_string(),
            "gemini-2.5-pro".to_string(),
            "gemini-2.5-flash".to_string(),
            "gemini-2.5-flash-lite".to_string(),
            "gemini-2.0-flash".to_string(),
            "gemini-2.0-flash-lite".to_string(),
            "gemma-4-31b-it".to_string(),
            "gemma-4-26b-a4b-it".to_string(),
            "gemma-3-27b-it".to_string(),
            "gemma-3-12b-it".to_string(),
            "gemma-3-4b-it".to_string(),
            "gemma-3-1b-it".to_string(),
        ]
    }

    fn context_window(&self, model: &str) -> Option<u32> {
        // Best-effort figures; exact limits are subject to change per Google's
        // release notes. Unknown models get a conservative default rather than
        // `None`, matching the OpenAI/Ollama providers' behavior.
        match model {
            "gemini-3-pro" | "gemini-2.5-pro" => Some(2_000_000),
            "gemini-2.5-flash" | "gemini-2.5-flash-lite" => Some(1_000_000),
            "gemini-2.0-flash" | "gemini-2.0-flash-lite" => Some(1_000_000),
            "gemma-4-31b-it" | "gemma-4-26b-a4b-it" => Some(128_000),
            "gemma-3-27b-it" | "gemma-3-12b-it" | "gemma-3-4b-it" => Some(128_000),
            "gemma-3-1b-it" => Some(32_000),
            _ => Some(32_000),
        }
    }

    fn calculate_cost(&self, model: &str, input_tokens: u32, output_tokens: u32) -> f64 {
        // Approximate cost per million tokens (paid tier, standard context).
        // Gemma models are free to use through the Gemini API as of this writing.
        let (input_cost, output_cost) = match model {
            "gemini-3-pro" | "gemini-2.5-pro" => (1.25, 10.0),
            "gemini-2.5-flash" => (0.30, 2.50),
            "gemini-2.5-flash-lite" => (0.10, 0.40),
            "gemini-2.0-flash" => (0.10, 0.40),
            "gemini-2.0-flash-lite" => (0.075, 0.30),
            _ if model.starts_with("gemma") => (0.0, 0.0),
            _ => return 0.0,
        };

        let input_cost_total = (input_tokens as f64 / 1_000_000.0) * input_cost;
        let output_cost_total = (output_tokens as f64 / 1_000_000.0) * output_cost;

        input_cost_total + output_cost_total
    }
}

// ============================================================================
// Gemini API Types
// ============================================================================

#[derive(Debug, Clone, Serialize)]
struct GeminiRequest {
    contents: Vec<GeminiContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system_instruction: Option<GeminiContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<GeminiTool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_config: Option<GeminiToolConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation_config: Option<GeminiGenerationConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GeminiContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<String>,
    parts: Vec<GeminiPart>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct GeminiPart {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    function_call: Option<GeminiFunctionCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    function_response: Option<GeminiFunctionResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_data: Option<GeminiInlineData>,
    /// Set by Gemini on response parts belonging to a thinking/reasoning trace.
    #[serde(skip_serializing_if = "Option::is_none")]
    thought: Option<bool>,
}

impl GeminiPart {
    fn text(text: String) -> Self {
        Self {
            text: Some(text),
            ..Default::default()
        }
    }

    fn with_inline_data(mut self, mime_type: String, data: String) -> Self {
        // Wrapped separately to keep `GeminiPart` construction simple above;
        // Gemini's inline_data part is `{"inlineData":{"mimeType","data"}}`.
        self.text = None;
        self.inline_data = Some(GeminiInlineData { mime_type, data });
        self
    }
}

/// Extension field kept out of the main struct body above for readability.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GeminiInlineData {
    mime_type: String,
    data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GeminiFunctionCall {
    name: String,
    #[serde(default)]
    args: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GeminiFunctionResponse {
    name: String,
    response: serde_json::Value,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct GeminiTool {
    function_declarations: Vec<GeminiFunctionDeclaration>,
}

#[derive(Debug, Clone, Serialize)]
struct GeminiFunctionDeclaration {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct GeminiToolConfig {
    function_calling_config: GeminiFunctionCallingConfig,
}

#[derive(Debug, Clone, Serialize)]
struct GeminiFunctionCallingConfig {
    mode: String,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
struct GeminiGenerationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_output_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_sequences: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_schema: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thinking_config: Option<GeminiThinkingConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seed: Option<u64>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct GeminiThinkingConfig {
    thinking_budget: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_thoughts: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GeminiResponse {
    #[serde(default)]
    candidates: Vec<GeminiCandidate>,
    #[serde(default)]
    usage_metadata: Option<GeminiUsageMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GeminiCandidate {
    #[serde(default)]
    content: Option<GeminiContent>,
    #[serde(default)]
    finish_reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct GeminiUsageMetadata {
    #[serde(default)]
    prompt_token_count: u32,
    #[serde(default)]
    candidates_token_count: u32,
    #[serde(default)]
    thoughts_token_count: u32,
}

#[derive(Debug, Clone, Deserialize)]
struct GeminiErrorResponse {
    error: GeminiError,
}

#[derive(Debug, Clone, Deserialize)]
struct GeminiError {
    message: String,
    #[serde(default)]
    status: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gemini_provider_creation() {
        let provider = GeminiProvider::new("test-key".to_string());
        assert_eq!(provider.name(), "gemini");
        assert_eq!(provider.default_model(), "gemini-2.5-flash");
        assert_eq!(provider.base_url, DEFAULT_GEMINI_API_URL);
    }

    #[test]
    fn test_custom_default_model() {
        let provider = GeminiProvider::new("test-key".to_string())
            .with_default_model("gemma-4-31b-it".to_string());
        assert_eq!(provider.default_model(), "gemma-4-31b-it");
    }

    #[test]
    fn test_supported_models_include_gemma() {
        let provider = GeminiProvider::new("test-key".to_string());
        let models = provider.supported_models();
        assert!(models.contains(&"gemma-4-31b-it".to_string()));
        assert!(models.contains(&"gemma-4-26b-a4b-it".to_string()));
        assert!(models.contains(&"gemini-2.5-pro".to_string()));
    }

    #[test]
    fn test_context_window() {
        let provider = GeminiProvider::new("test-key".to_string());
        assert_eq!(provider.context_window("gemini-2.5-pro"), Some(2_000_000));
        assert_eq!(provider.context_window("gemma-4-31b-it"), Some(128_000));
        assert_eq!(provider.context_window("unknown-model"), Some(32_000));
    }

    #[test]
    fn test_gemma_cost_is_free() {
        let provider = GeminiProvider::new("test-key".to_string());
        assert_eq!(
            provider.calculate_cost("gemma-4-31b-it", 10_000, 10_000),
            0.0
        );
    }

    #[test]
    fn test_calculate_cost_gemini_flash() {
        let provider = GeminiProvider::new("test-key".to_string());
        // 1M input + 1M output tokens on gemini-2.5-flash: 0.30 + 2.50 = 2.80
        let cost = provider.calculate_cost("gemini-2.5-flash", 1_000_000, 1_000_000);
        assert!((cost - 2.80).abs() < 0.0001);
    }

    #[test]
    fn test_role_mapping() {
        assert_eq!(gemini_role(&Role::User), "user");
        assert_eq!(gemini_role(&Role::Assistant), "model");
        assert_eq!(gemini_role(&Role::System), "user");
    }

    #[test]
    fn test_to_gemini_request_maps_system_and_tools() {
        let provider = GeminiProvider::new("test-key".to_string());
        let tool = Tool {
            name: "read_file".to_string(),
            description: "Read a file".to_string(),
            input_schema: serde_json::json!({"type": "object"}),
        };
        let request = LLMRequest::new("gemini-2.5-flash", vec![Message::user("hi")])
            .with_system("You are helpful")
            .with_tools(vec![tool]);

        let gemini_req = provider.to_gemini_request(&request);
        assert!(gemini_req.system_instruction.is_some());
        let tools = gemini_req.tools.expect("tools must be set");
        assert_eq!(tools[0].function_declarations[0].name, "read_file");
        assert_eq!(gemini_req.contents.len(), 1);
        assert_eq!(gemini_req.contents[0].role.as_deref(), Some("user"));
    }

    #[test]
    fn test_to_gemini_request_tool_result_uses_function_name() {
        let provider = GeminiProvider::new("test-key".to_string());
        let messages = vec![
            Message {
                role: Role::Assistant,
                content: vec![ContentBlock::ToolUse {
                    id: "call_1".to_string(),
                    name: "read_file".to_string(),
                    input: serde_json::json!({"path": "a.txt"}),
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
        ];
        let request = LLMRequest::new("gemini-2.5-flash", messages);
        let gemini_req = provider.to_gemini_request(&request);

        assert_eq!(gemini_req.contents.len(), 2);
        let function_response = gemini_req.contents[1].parts[0]
            .function_response
            .as_ref()
            .expect("expected functionResponse part");
        assert_eq!(function_response.name, "read_file");
    }

    #[test]
    fn test_from_gemini_response_maps_tool_use() {
        let provider = GeminiProvider::new("test-key".to_string());
        let response = GeminiResponse {
            candidates: vec![GeminiCandidate {
                content: Some(GeminiContent {
                    role: Some("model".to_string()),
                    parts: vec![GeminiPart {
                        function_call: Some(GeminiFunctionCall {
                            name: "read_file".to_string(),
                            args: serde_json::json!({"path": "a.txt"}),
                        }),
                        ..Default::default()
                    }],
                }),
                finish_reason: Some("STOP".to_string()),
            }],
            usage_metadata: Some(GeminiUsageMetadata {
                prompt_token_count: 10,
                candidates_token_count: 5,
                thoughts_token_count: 0,
            }),
        };

        let llm_response = provider.from_gemini_response(response, "gemini-2.5-flash");
        assert_eq!(llm_response.stop_reason, Some(StopReason::ToolUse));
        assert_eq!(llm_response.usage.input_tokens, 10);
        assert_eq!(llm_response.usage.output_tokens, 5);
        match &llm_response.content[0] {
            ContentBlock::ToolUse { name, .. } => assert_eq!(name, "read_file"),
            other => panic!("expected ToolUse, got {:?}", other),
        }
    }

    #[test]
    fn test_from_gemini_response_maps_thinking() {
        let provider = GeminiProvider::new("test-key".to_string());
        let response = GeminiResponse {
            candidates: vec![GeminiCandidate {
                content: Some(GeminiContent {
                    role: Some("model".to_string()),
                    parts: vec![
                        GeminiPart {
                            text: Some("reasoning...".to_string()),
                            thought: Some(true),
                            ..Default::default()
                        },
                        GeminiPart {
                            text: Some("final answer".to_string()),
                            ..Default::default()
                        },
                    ],
                }),
                finish_reason: Some("STOP".to_string()),
            }],
            usage_metadata: None,
        };

        let llm_response = provider.from_gemini_response(response, "gemini-2.5-pro");
        assert!(matches!(
            llm_response.content[0],
            ContentBlock::Thinking { .. }
        ));
        assert!(matches!(llm_response.content[1], ContentBlock::Text { .. }));
    }

    #[test]
    fn test_thinking_config_forwarded() {
        let provider = GeminiProvider::new("test-key".to_string());
        let request =
            LLMRequest::new("gemini-2.5-pro", vec![Message::user("hi")]).with_thinking(4096);
        let gemini_req = provider.to_gemini_request(&request);
        let tc = gemini_req
            .generation_config
            .unwrap()
            .thinking_config
            .expect("thinking config must be set");
        assert_eq!(tc.thinking_budget, 4096);
        assert_eq!(tc.include_thoughts, Some(true));
    }
}
