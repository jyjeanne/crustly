//! Anthropic (Claude) Provider Implementation
//!
//! Implements the Provider trait for Anthropic's Claude models.
//!
//! ## Supported Models
//! - claude-3-opus-20240229
//! - claude-3-sonnet-20240229
//! - claude-3-5-sonnet-20240620
//! - claude-3-haiku-20240307

use super::error::{ProviderError, Result};
use super::r#trait::{Provider, ProviderStream};
use super::types::*;
use async_trait::async_trait;
use futures::stream::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

const ANTHROPIC_API_URL: &str = "https://api.anthropic.com/v1/messages";
const ANTHROPIC_VERSION: &str = "2023-06-01";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(120); // Total request timeout
const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(10); // Connection timeout
const DEFAULT_POOL_IDLE_TIMEOUT: Duration = Duration::from_secs(90); // Keep connections alive

/// Anthropic provider for Claude models
#[derive(Clone)]
pub struct AnthropicProvider {
    api_key: String,
    client: Client,
}

impl AnthropicProvider {
    /// Create a new Anthropic provider
    pub fn new(api_key: String) -> Self {
        let client = Client::builder()
            .timeout(DEFAULT_TIMEOUT) // Total request timeout (including streaming)
            .connect_timeout(DEFAULT_CONNECT_TIMEOUT) // Connection establishment timeout
            .pool_idle_timeout(DEFAULT_POOL_IDLE_TIMEOUT) // Keep connections in pool
            .pool_max_idle_per_host(2) // Max idle connections per host
            .build()
            .expect("Failed to create HTTP client");

        Self { api_key, client }
    }

    /// Create with custom HTTP client
    pub fn with_client(api_key: String, client: Client) -> Self {
        Self { api_key, client }
    }

    /// Build request headers
    fn headers(&self) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "x-api-key",
            self.api_key.parse().expect("Invalid API key format"),
        );
        headers.insert(
            "anthropic-version",
            ANTHROPIC_VERSION.parse().expect("Invalid version"),
        );
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );
        headers
    }

    /// Convert our generic request to Anthropic-specific format
    fn to_anthropic_request(&self, request: LLMRequest) -> AnthropicRequest {
        AnthropicRequest {
            model: request.model,
            messages: request.messages,
            system: request.system,
            max_tokens: request.max_tokens.unwrap_or(4096),
            temperature: request.temperature,
            tools: request.tools,
            stream: Some(request.stream),
            metadata: request.metadata,
        }
    }

    /// Convert Anthropic response to our generic format
    #[allow(clippy::wrong_self_convention)]
    fn from_anthropic_response(&self, response: AnthropicResponse) -> LLMResponse {
        LLMResponse {
            id: response.id,
            model: response.model,
            content: response.content,
            stop_reason: response.stop_reason,
            usage: response.usage,
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
        if let Ok(error_body) = response.json::<AnthropicError>().await {
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
                    error_type: Some(error_body.error.error_type),
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
impl Provider for AnthropicProvider {
    async fn complete(&self, request: LLMRequest) -> Result<LLMResponse> {
        use super::retry::{retry_with_backoff, RetryConfig};

        let anthropic_request = self.to_anthropic_request(request);
        let retry_config = RetryConfig::default();

        // Retry the entire API call with exponential backoff
        retry_with_backoff(
            || async {
                let response = self
                    .client
                    .post(ANTHROPIC_API_URL)
                    .headers(self.headers())
                    .json(&anthropic_request)
                    .send()
                    .await?;

                if !response.status().is_success() {
                    return Err(self.handle_error(response).await);
                }

                let anthropic_response: AnthropicResponse = response.json().await?;
                Ok(self.from_anthropic_response(anthropic_response))
            },
            &retry_config,
        )
        .await
    }

    async fn stream(&self, request: LLMRequest) -> Result<ProviderStream> {
        use super::retry::{retry_with_backoff, RetryConfig};

        let mut anthropic_request = self.to_anthropic_request(request);
        anthropic_request.stream = Some(true);
        let retry_config = RetryConfig::default();

        // Retry the stream connection establishment
        let response = retry_with_backoff(
            || async {
                let response = self
                    .client
                    .post(ANTHROPIC_API_URL)
                    .headers(self.headers())
                    .json(&anthropic_request)
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
                .and_then(|chunk| {
                    // Parse SSE format: "data: {json}\n\n"
                    let text = String::from_utf8_lossy(&chunk);

                    // Split by SSE event delimiter
                    for line in text.lines() {
                        if let Some(json_str) = line.strip_prefix("data: ") {
                            if json_str == "[DONE]" {
                                continue;
                            }

                            // Parse the JSON event
                            return serde_json::from_str::<StreamEvent>(json_str)
                                .map_err(ProviderError::JsonError);
                        }
                    }

                    // Skip non-data lines (e.g., "event: message_start")
                    Ok(StreamEvent::Ping)
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
        true
    }

    fn name(&self) -> &str {
        "anthropic"
    }

    fn default_model(&self) -> &str {
        "claude-3-5-sonnet-20240620"
    }

    fn supported_models(&self) -> Vec<String> {
        vec![
            "claude-3-opus-20240229".to_string(),
            "claude-3-sonnet-20240229".to_string(),
            "claude-3-5-sonnet-20240620".to_string(),
            "claude-3-haiku-20240307".to_string(),
        ]
    }

    fn context_window(&self, model: &str) -> Option<u32> {
        match model {
            "claude-3-opus-20240229" => Some(200_000),
            "claude-3-sonnet-20240229" => Some(200_000),
            "claude-3-5-sonnet-20240620" => Some(200_000),
            "claude-3-haiku-20240307" => Some(200_000),
            _ => None,
        }
    }

    fn calculate_cost(&self, model: &str, input_tokens: u32, output_tokens: u32) -> f64 {
        // Costs per million tokens (as of 2024)
        let (input_cost, output_cost) = match model {
            "claude-3-opus-20240229" => (15.0, 75.0),
            "claude-3-sonnet-20240229" => (3.0, 15.0),
            "claude-3-5-sonnet-20240620" => (3.0, 15.0),
            "claude-3-haiku-20240307" => (0.25, 1.25),
            _ => return 0.0,
        };

        let input_cost_total = (input_tokens as f64 / 1_000_000.0) * input_cost;
        let output_cost_total = (output_tokens as f64 / 1_000_000.0) * output_cost;

        input_cost_total + output_cost_total
    }
}

// Anthropic-specific request format
#[derive(Debug, Serialize)]
struct AnthropicRequest {
    model: String,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    max_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<Tool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
}

// Anthropic-specific response format
#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    id: String,
    model: String,
    content: Vec<ContentBlock>,
    stop_reason: Option<StopReason>,
    usage: TokenUsage,
}

// Anthropic error format
#[derive(Debug, Deserialize)]
struct AnthropicError {
    error: AnthropicErrorDetail,
}

#[derive(Debug, Deserialize)]
struct AnthropicErrorDetail {
    #[serde(rename = "type")]
    error_type: String,
    message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anthropic_provider_creation() {
        let provider = AnthropicProvider::new("test-key".to_string());
        assert_eq!(provider.name(), "anthropic");
        assert_eq!(provider.default_model(), "claude-3-5-sonnet-20240620");
    }

    #[test]
    fn test_supported_models() {
        let provider = AnthropicProvider::new("test-key".to_string());
        let models = provider.supported_models();
        assert!(models.contains(&"claude-3-opus-20240229".to_string()));
        assert!(models.contains(&"claude-3-5-sonnet-20240620".to_string()));
    }

    #[test]
    fn test_context_window() {
        let provider = AnthropicProvider::new("test-key".to_string());
        assert_eq!(
            provider.context_window("claude-3-opus-20240229"),
            Some(200_000)
        );
        assert_eq!(provider.context_window("unknown-model"), None);
    }

    #[test]
    fn test_cost_calculation() {
        let provider = AnthropicProvider::new("test-key".to_string());

        // Test Opus pricing (most expensive)
        let cost = provider.calculate_cost("claude-3-opus-20240229", 1_000_000, 1_000_000);
        assert_eq!(cost, 90.0); // $15 input + $75 output

        // Test Haiku pricing (least expensive)
        let cost = provider.calculate_cost("claude-3-haiku-20240307", 1_000_000, 1_000_000);
        assert_eq!(cost, 1.5); // $0.25 input + $1.25 output
    }

    #[test]
    fn test_capabilities() {
        let provider = AnthropicProvider::new("test-key".to_string());
        assert!(provider.supports_streaming());
        assert!(provider.supports_tools());
        assert!(provider.supports_vision());
    }
}
