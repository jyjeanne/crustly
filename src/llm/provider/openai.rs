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
use futures::stream::StreamExt;
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
        }
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
                content: system,
            });
        }

        // Add conversation messages
        for msg in request.messages {
            let role = match msg.role {
                Role::User => "user",
                Role::Assistant => "assistant",
                Role::System => "system",
            };

            // Extract text from content blocks
            let content: String = msg
                .content
                .iter()
                .filter_map(|block| {
                    if let ContentBlock::Text { text } = block {
                        Some(text.clone())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");

            messages.push(OpenAIMessage {
                role: role.to_string(),
                content,
            });
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

        OpenAIRequest {
            model: request.model,
            messages,
            temperature: request.temperature,
            max_tokens: request.max_tokens,
            stream: Some(request.stream),
            tools,
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
                    content: "".to_string(),
                },
                finish_reason: Some("error".to_string()),
            });

        // Convert content to content blocks
        let mut content_blocks = Vec::new();
        if !choice.message.content.is_empty() {
            content_blocks.push(ContentBlock::Text {
                text: choice.message.content,
            });
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
                input_tokens: response.usage.prompt_tokens,
                output_tokens: response.usage.completion_tokens,
            },
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

        let openai_request = self.to_openai_request(request);
        let retry_config = RetryConfig::default();

        let tool_count = openai_request.tools.as_ref().map(|t| t.len()).unwrap_or(0);
        tracing::debug!(
            "Sending OpenAI request to {} with model {} and {} tools",
            self.base_url,
            openai_request.model,
            tool_count
        );
        if tool_count == 0 {
            tracing::warn!("OpenAI request has NO tools - LLM won't know about file/bash operations!");
        }

        // Retry the entire API call with exponential backoff
        retry_with_backoff(
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

                let openai_response: OpenAIResponse = response.json().await?;
                Ok(self.from_openai_response(openai_response))
            },
            &retry_config,
        )
        .await
    }

    async fn stream(&self, request: LLMRequest) -> Result<ProviderStream> {
        use super::retry::{retry_with_backoff, RetryConfig};

        let mut openai_request = self.to_openai_request(request);
        openai_request.stream = Some(true);
        let retry_config = RetryConfig::default();

        tracing::debug!(
            "Starting OpenAI stream to {} with model {}",
            self.base_url,
            openai_request.model
        );

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

        // Parse Server-Sent Events stream
        let byte_stream = response.bytes_stream();
        let event_stream = byte_stream.map(|chunk_result| {
            chunk_result
                .map_err(|e| ProviderError::StreamError(e.to_string()))
                .map(|chunk| {
                    let text = String::from_utf8_lossy(&chunk);

                    // Parse SSE format: "data: {...}\n\n"
                    for line in text.lines() {
                        if let Some(json_str) = line.strip_prefix("data: ") {
                            // Check for stream end
                            if json_str == "[DONE]" {
                                return StreamEvent::MessageStop;
                            }

                            // Parse JSON chunk
                            if let Ok(chunk) = serde_json::from_str::<OpenAIStreamChunk>(json_str) {
                                if let Some(choice) = chunk.choices.first() {
                                    if let Some(ref delta) = choice.delta {
                                        if let Some(ref content) = delta.content {
                                            if !content.is_empty() {
                                                return StreamEvent::ContentBlockDelta {
                                                    index: 0,
                                                    delta: ContentDelta::TextDelta {
                                                        text: content.clone(),
                                                    },
                                                };
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Skip non-data lines
                    StreamEvent::Ping
                })
        });

        Ok(Box::pin(event_stream))
    }

    fn supports_streaming(&self) -> bool {
        true
    }

    fn supports_tools(&self) -> bool {
        true
    }

    fn supports_vision(&self) -> bool {
        // Only GPT-4 Vision models support vision
        false
    }

    fn name(&self) -> &str {
        "openai"
    }

    fn default_model(&self) -> &str {
        "gpt-4-turbo-preview"
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
            _ => None,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<OpenAITool>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OpenAIMessage {
    role: String,
    content: String,
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
    usage: OpenAIUsage,
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
    choices: Vec<OpenAIStreamChoice>,
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
        assert_eq!(provider.context_window("unknown"), None);
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
