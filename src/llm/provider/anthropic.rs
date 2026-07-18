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

    /// Build request headers.
    ///
    /// The API key is user/config-supplied (env var, keyring, file) and
    /// commonly picks up a trailing newline or other whitespace (e.g.
    /// `export KEY=$(cat key.txt)` without `-n`); `HeaderValue::parse`
    /// rejects any byte outside the printable-ASCII header range, so an
    /// `.expect()` here used to crash the whole process on the very first
    /// request. Trim first (covers the common case) and return a proper
    /// error for anything still invalid instead of panicking.
    fn headers(&self) -> Result<reqwest::header::HeaderMap> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "x-api-key",
            self.api_key
                .trim()
                .parse()
                .map_err(|_| ProviderError::InvalidApiKey)?,
        );
        headers.insert(
            "anthropic-version",
            ANTHROPIC_VERSION
                .parse()
                .expect("static version string is always a valid header value"),
        );
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json"
                .parse()
                .expect("static content-type string is always a valid header value"),
        );
        Ok(headers)
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
            thinking: request.thinking,
        }
    }

    /// Convert Anthropic response to our generic format
    #[allow(clippy::wrong_self_convention)]
    fn from_anthropic_response(&self, response: AnthropicResponse) -> LLMResponse {
        let cache_metrics = if response.usage.cache_read_input_tokens > 0
            || response.usage.cache_creation_input_tokens > 0
        {
            Some(super::types::CacheMetrics {
                read_tokens: response.usage.cache_read_input_tokens,
                creation_tokens: response.usage.cache_creation_input_tokens,
            })
        } else {
            None
        };

        LLMResponse {
            id: response.id,
            model: response.model,
            content: response.content,
            stop_reason: response.stop_reason,
            usage: TokenUsage {
                input_tokens: response.usage.input_tokens,
                output_tokens: response.usage.output_tokens,
            },
            cache_metrics,
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

        let model = request.model.clone();
        let message_count = request.messages.len();
        tracing::info!(
            "Anthropic API request: model={}, messages={}, max_tokens={}",
            model,
            message_count,
            request.max_tokens.unwrap_or(4096)
        );

        let anthropic_request = self.to_anthropic_request(request);
        let retry_config = RetryConfig::default();

        // Retry the entire API call with exponential backoff
        let result = retry_with_backoff(
            || async {
                tracing::debug!("Sending request to Anthropic API");
                let response = self
                    .client
                    .post(ANTHROPIC_API_URL)
                    .headers(self.headers()?)
                    .json(&anthropic_request)
                    .send()
                    .await?;

                let status = response.status();
                tracing::debug!("Anthropic API response status: {}", status);

                if !status.is_success() {
                    return Err(self.handle_error(response).await);
                }

                let anthropic_response: AnthropicResponse = response.json().await?;
                let llm_response = self.from_anthropic_response(anthropic_response);

                tracing::info!(
                    "Anthropic API response: input_tokens={}, output_tokens={}, stop_reason={:?}",
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
            tracing::error!("Anthropic API request failed: {}", e);
        }

        result
    }

    async fn stream(&self, request: LLMRequest) -> Result<ProviderStream> {
        use super::retry::{retry_with_backoff, RetryConfig};

        let model = request.model.clone();
        let message_count = request.messages.len();
        tracing::info!(
            "Anthropic streaming request: model={}, messages={}",
            model,
            message_count
        );

        let mut anthropic_request = self.to_anthropic_request(request);
        anthropic_request.stream = Some(true);
        let retry_config = RetryConfig::default();

        // Retry the stream connection establishment
        let response = retry_with_backoff(
            || async {
                let response = self
                    .client
                    .post(ANTHROPIC_API_URL)
                    .headers(self.headers()?)
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

        Ok(Box::pin(parse_anthropic_sse_stream(
            response.bytes_stream(),
        )))
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
            // `ModelRouter::default_anthropic()` (router.rs) routes real
            // traffic to dated model IDs newer than the exact matches above
            // (e.g. `claude-sonnet-4-6`), which fell through to `_ => 0.0`
            // and silently recorded every session's cost as free. Every new
            // model release would otherwise need a matching update here, so
            // fall back to family-tier pricing by substring instead of
            // requiring an exact, ever-growing list. Tier pricing is
            // consistently ordered opus > sonnet > haiku across Anthropic's
            // lineup, so this is an approximation, not exact - logged so a
            // stale/wrong tier price is discoverable rather than silent.
            _ if model.contains("opus") => (15.0, 75.0),
            _ if model.contains("sonnet") => (3.0, 15.0),
            _ if model.contains("haiku") => (0.25, 1.25),
            _ => {
                tracing::warn!(
                    "Unknown Anthropic model '{}' for cost calculation - recording $0.00",
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

/// Turn a raw HTTP byte stream into a stream of parsed Anthropic SSE events.
///
/// A 1:1 `.map()` over network chunks used to sit here, which is wrong on
/// both sides: a single chunk can carry more than one `"data: {json}\n\n"`
/// event (only the first was ever returned, silently dropping the rest),
/// and a single event's JSON can be split across two chunks (a normal
/// TCP/HTTP occurrence), which failed to parse and aborted the whole
/// stream. `scan` carries a byte buffer across chunks so a line is only
/// decoded once a `\n` has actually arrived, and `flat_map` lets one chunk
/// yield any number of events (including zero, for a chunk that only
/// advanced the buffer). Factored out of `stream()` so it can be exercised
/// directly with a synthetic chunk sequence, without a mock HTTP server.
fn parse_anthropic_sse_stream(
    byte_stream: impl futures::Stream<Item = reqwest::Result<bytes::Bytes>> + Send + 'static,
) -> impl futures::Stream<Item = Result<StreamEvent>> + Send + 'static {
    byte_stream
        .scan(Vec::<u8>::new(), |buf, chunk_result| {
            let events: Vec<Result<StreamEvent>> = match chunk_result {
                Err(e) => vec![Err(ProviderError::StreamError(e.to_string()))],
                Ok(chunk) => {
                    buf.extend_from_slice(&chunk);
                    let mut events = Vec::new();

                    // Splitting on the raw `\n` byte is safe even for UTF-8
                    // text containing multi-byte characters: 0x0A never
                    // appears as a continuation byte, so this never cuts a
                    // character in half.
                    while let Some(pos) = buf.iter().position(|&b| b == b'\n') {
                        let line_bytes: Vec<u8> = buf.drain(..=pos).collect();
                        let decoded = String::from_utf8_lossy(&line_bytes);
                        let line = decoded.trim_end_matches(['\r', '\n']);

                        if let Some(json_str) = line.strip_prefix("data: ") {
                            if json_str == "[DONE]" {
                                tracing::trace!("Stream completed with [DONE] marker");
                                continue;
                            }
                            match serde_json::from_str::<StreamEvent>(json_str) {
                                Ok(event) => events.push(Ok(event)),
                                Err(e) => {
                                    tracing::warn!(
                                        "Failed to parse SSE event JSON: {}. Data: {}",
                                        e,
                                        json_str.chars().take(200).collect::<String>()
                                    );
                                    events.push(Err(ProviderError::JsonError(e)));
                                }
                            }
                        } else if !line.trim().is_empty()
                            && !line.starts_with("event:")
                            && !line.starts_with("id:")
                            && !line.starts_with("retry:")
                        {
                            tracing::debug!("Unexpected SSE line format: {}", line);
                        }
                    }

                    events
                }
            };
            futures::future::ready(Some(events))
        })
        .flat_map(futures::stream::iter)
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
    /// Extended thinking (claude-3-7-sonnet+ only). When set, temperature must be 1.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    thinking: Option<ThinkingConfig>,
}

// Anthropic-specific response format
#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    id: String,
    model: String,
    content: Vec<ContentBlock>,
    stop_reason: Option<StopReason>,
    usage: AnthropicTokenUsage,
}

/// Anthropic's extended usage object includes prompt-cache fields.
#[derive(Debug, Deserialize)]
struct AnthropicTokenUsage {
    input_tokens: u32,
    output_tokens: u32,
    #[serde(default)]
    cache_read_input_tokens: u32,
    #[serde(default)]
    cache_creation_input_tokens: u32,
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
    fn test_cost_calculation_falls_back_to_family_tier_for_unlisted_model_ids() {
        let provider = AnthropicProvider::new("test-key".to_string());

        // Newer dated model IDs (e.g. what `ModelRouter::default_anthropic`
        // routes to) aren't in the exact-match table, but should still be
        // priced via the opus/sonnet/haiku substring fallback instead of
        // silently returning $0.00.
        assert_eq!(
            provider.calculate_cost("claude-opus-4-7", 1_000_000, 1_000_000),
            90.0
        );
        assert_eq!(
            provider.calculate_cost("claude-sonnet-4-6", 1_000_000, 1_000_000),
            18.0
        );
        assert_eq!(
            provider.calculate_cost("claude-haiku-4-5-20251001", 1_000_000, 1_000_000),
            1.5
        );
    }

    #[test]
    fn test_cost_calculation_unknown_model_family_returns_zero() {
        let provider = AnthropicProvider::new("test-key".to_string());
        assert_eq!(
            provider.calculate_cost("totally-unknown-model", 1000, 1000),
            0.0
        );
    }

    /// Regression: the old `.map()` over network chunks only ever returned
    /// the *first* `data:` line found in a chunk. Two complete SSE events
    /// delivered together in a single network read used to silently drop
    /// the second one.
    #[tokio::test]
    async fn sse_stream_yields_every_event_in_a_single_chunk() {
        let chunk = concat!(
            "data: {\"type\":\"content_block_stop\",\"index\":0}\n\n",
            "data: {\"type\":\"content_block_stop\",\"index\":1}\n\n",
        );
        let byte_stream = futures::stream::iter(vec![Ok::<bytes::Bytes, reqwest::Error>(
            bytes::Bytes::from(chunk),
        )]);

        let events: Vec<StreamEvent> = parse_anthropic_sse_stream(byte_stream)
            .map(|r| r.expect("event must parse"))
            .collect()
            .await;

        let indices: Vec<usize> = events
            .iter()
            .map(|e| match e {
                StreamEvent::ContentBlockStop { index } => *index,
                other => panic!("expected ContentBlockStop, got {other:?}"),
            })
            .collect();
        assert_eq!(indices, vec![0, 1], "both events in the chunk must survive");
    }

    /// Regression: the old code decoded each network chunk independently,
    /// so a single SSE event's JSON split across two TCP reads (a normal
    /// occurrence, not a corner case) failed to parse as JSON and aborted
    /// the whole stream with a `JsonError`.
    #[tokio::test]
    async fn sse_stream_reassembles_an_event_split_across_chunks() {
        let full_line = "data: {\"type\":\"content_block_stop\",\"index\":7}\n\n";
        let split_at = full_line.find("\"index\"").unwrap();
        let (first_half, second_half) = full_line.split_at(split_at);

        let byte_stream = futures::stream::iter(vec![
            Ok::<bytes::Bytes, reqwest::Error>(bytes::Bytes::from(first_half.to_string())),
            Ok::<bytes::Bytes, reqwest::Error>(bytes::Bytes::from(second_half.to_string())),
        ]);

        let events: Vec<StreamEvent> = parse_anthropic_sse_stream(byte_stream)
            .map(|r| r.expect("event must parse even though its JSON arrived in two chunks"))
            .collect()
            .await;

        assert_eq!(events.len(), 1);
        match &events[0] {
            StreamEvent::ContentBlockStop { index } => assert_eq!(*index, 7),
            other => panic!("expected ContentBlockStop, got {other:?}"),
        }
    }

    #[test]
    fn test_capabilities() {
        let provider = AnthropicProvider::new("test-key".to_string());
        assert!(provider.supports_streaming());
        assert!(provider.supports_tools());
        assert!(provider.supports_vision());
    }
}
