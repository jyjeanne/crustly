//! Core types for LLM provider abstraction
//!
//! Defines common types used across all LLM providers.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Role of a message in the conversation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    /// User message
    User,
    /// Assistant message
    Assistant,
    /// System message (not all providers support this)
    System,
}

/// A message in the conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Role of the message sender
    pub role: Role,
    /// Content blocks of the message
    pub content: Vec<ContentBlock>,
}

impl Message {
    /// Create a new user message with text content
    pub fn user(text: impl Into<String>) -> Self {
        Self {
            role: Role::User,
            content: vec![ContentBlock::Text { text: text.into() }],
        }
    }

    /// Create a new assistant message with text content
    pub fn assistant(text: impl Into<String>) -> Self {
        Self {
            role: Role::Assistant,
            content: vec![ContentBlock::Text { text: text.into() }],
        }
    }

    /// Create a new system message with text content
    pub fn system(text: impl Into<String>) -> Self {
        Self {
            role: Role::System,
            content: vec![ContentBlock::Text { text: text.into() }],
        }
    }
}

/// Content block in a message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlock {
    /// Plain text content
    Text { text: String },
    /// Image content (base64 or URL)
    Image { source: ImageSource },
    /// Tool use request from assistant
    ToolUse {
        id: String,
        name: String,
        input: serde_json::Value,
    },
    /// Tool result from user
    ToolResult {
        tool_use_id: String,
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_error: Option<bool>,
    },
    /// Extended thinking block (Anthropic claude-3-7-sonnet+).
    /// Not rendered as assistant text by default.
    Thinking { thinking: String },
}

/// Image source for image content blocks
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ImageSource {
    /// Base64-encoded image
    Base64 { media_type: String, data: String },
    /// Image URL
    Url { url: String },
}

/// Extended thinking configuration (Anthropic claude-3-7-sonnet+).
/// When enabled, temperature MUST be 1.0 — enforced by `with_thinking()`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThinkingConfig {
    /// Always "enabled".
    pub r#type: String,
    /// Token budget for the internal reasoning trace.
    pub budget_tokens: u32,
}

/// LLM request parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMRequest {
    /// Model to use
    pub model: String,
    /// Conversation messages
    pub messages: Vec<Message>,
    /// System prompt (if supported)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    /// Available tools
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    /// Temperature (0.0-1.0); forced to 1.0 when thinking is active
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Nucleus sampling threshold (0.0-1.0). Limits token selection to the
    /// top-p probability mass. Mutually exclusive with `temperature` in most
    /// providers — use one or the other.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    /// Random seed for reproducible outputs (supported by Ollama and OpenAI).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u64>,
    /// Stop sequences — generation halts when any of these strings are produced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    /// Penalises tokens by how often they have already appeared (−2.0..2.0).
    /// Reduces repetition; forwarded to OpenAI-compatible backends.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    /// Penalises tokens that have appeared at all (−2.0..2.0).
    /// Encourages topic diversity; forwarded to OpenAI-compatible backends.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    /// Response format override (e.g. `{"type":"json_object"}` for JSON mode,
    /// or a full JSON Schema for structured outputs).  Forwarded verbatim to
    /// OpenAI-compatible backends; ignored by Anthropic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<serde_json::Value>,
    /// Maximum tokens to generate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    /// Whether to stream the response
    #[serde(skip)]
    pub stream: bool,
    /// Additional metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    /// Extended thinking configuration (Anthropic only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking: Option<ThinkingConfig>,
}

impl LLMRequest {
    /// Create a new LLM request
    pub fn new(model: impl Into<String>, messages: Vec<Message>) -> Self {
        Self {
            model: model.into(),
            messages,
            system: None,
            tools: None,
            temperature: None,
            top_p: None,
            seed: None,
            stop: None,
            frequency_penalty: None,
            presence_penalty: None,
            response_format: None,
            max_tokens: None,
            stream: false,
            metadata: None,
            thinking: None,
        }
    }

    /// Enable extended thinking with the given token budget.
    /// Forces temperature to 1.0 per Anthropic API requirements.
    pub fn with_thinking(mut self, budget_tokens: u32) -> Self {
        if budget_tokens > 0 {
            self.thinking = Some(ThinkingConfig {
                r#type: "enabled".to_string(),
                budget_tokens,
            });
            self.temperature = Some(1.0);
        }
        self
    }

    /// Set system prompt
    pub fn with_system(mut self, system: impl Into<String>) -> Self {
        self.system = Some(system.into());
        self
    }

    /// Set tools
    pub fn with_tools(mut self, tools: Vec<Tool>) -> Self {
        self.tools = Some(tools);
        self
    }

    /// Set temperature
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Set top_p (nucleus sampling threshold, 0.0–1.0).
    /// Use instead of `temperature`, not alongside it.
    pub fn with_top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }

    /// Set a random seed for reproducible outputs.
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Set stop sequences — generation halts at the first match.
    pub fn with_stop(mut self, stop: Vec<String>) -> Self {
        self.stop = Some(stop);
        self
    }

    /// Set frequency penalty (−2.0..2.0). Reduces token repetition.
    pub fn with_frequency_penalty(mut self, penalty: f32) -> Self {
        self.frequency_penalty = Some(penalty);
        self
    }

    /// Set presence penalty (−2.0..2.0). Encourages topic diversity.
    pub fn with_presence_penalty(mut self, penalty: f32) -> Self {
        self.presence_penalty = Some(penalty);
        self
    }

    /// Set response format (e.g. JSON mode or a JSON Schema).
    /// Pass `serde_json::json!({"type":"json_object"})` for JSON mode.
    pub fn with_response_format(mut self, format: serde_json::Value) -> Self {
        self.response_format = Some(format);
        self
    }

    /// Set max tokens
    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    /// Enable streaming
    pub fn with_streaming(mut self) -> Self {
        self.stream = true;
        self
    }
}

/// Tool definition for LLM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    /// Tool name
    pub name: String,
    /// Tool description
    pub description: String,
    /// Input schema (JSON Schema)
    pub input_schema: serde_json::Value,
}

/// Prompt cache metrics reported by Anthropic.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CacheMetrics {
    /// Tokens read from the prompt cache (saved cost).
    pub read_tokens: u32,
    /// Tokens written to create a new cache entry.
    pub creation_tokens: u32,
}

impl CacheMetrics {
    /// Fraction of input tokens that were served from cache (0.0–1.0).
    pub fn hit_rate(&self) -> f32 {
        let total = self.read_tokens + self.creation_tokens;
        if total == 0 {
            0.0
        } else {
            self.read_tokens as f32 / total as f32
        }
    }
}

/// LLM response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMResponse {
    /// Response ID
    pub id: String,
    /// Model used
    pub model: String,
    /// Content blocks
    pub content: Vec<ContentBlock>,
    /// Stop reason
    pub stop_reason: Option<StopReason>,
    /// Token usage
    pub usage: TokenUsage,
    /// Optional prompt cache metrics (Anthropic only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_metrics: Option<CacheMetrics>,
    /// Optional runtime performance metrics (local inference backends, e.g. Ollama).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perf_metrics: Option<PerfMetrics>,
}

/// Runtime performance metrics reported by local inference backends.
///
/// `None` for providers that don't expose this level of detail (Anthropic,
/// OpenAI, Qwen, Azure) — purely additive, no behavior change for them.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PerfMetrics {
    /// Time to load/warm the model into memory (ms). `Some(0)` when the
    /// model was already resident (warm start).
    pub load_duration_ms: Option<u64>,
    /// Prefill duration — time spent evaluating the input prompt (ms).
    pub prompt_eval_duration_ms: Option<u64>,
    /// Generation duration — time spent producing the output (ms).
    pub eval_duration_ms: Option<u64>,
    /// Total wall-clock duration for the request (ms).
    pub total_duration_ms: Option<u64>,
    /// `true` if the model was already loaded (warm start), `false` if it
    /// had to be loaded first (cold start), `None` if unknown/unsupported.
    pub model_was_loaded: Option<bool>,
}

impl PerfMetrics {
    /// Generation throughput in tokens/second, derived from the output
    /// token count and the measured generation duration.
    pub fn tokens_per_second(&self, output_tokens: u32) -> Option<f64> {
        let ms = self.eval_duration_ms?;
        (ms > 0).then(|| output_tokens as f64 / (ms as f64 / 1000.0))
    }
}

/// Reason why the model stopped generating
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StopReason {
    /// Natural end of response
    EndTurn,
    /// Hit max tokens
    MaxTokens,
    /// Stop sequence encountered
    StopSequence,
    /// Tool use requested
    ToolUse,
}

/// Token usage information
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TokenUsage {
    /// Input tokens
    pub input_tokens: u32,
    /// Output tokens
    pub output_tokens: u32,
}

impl TokenUsage {
    /// Total tokens used
    pub fn total(&self) -> u32 {
        self.input_tokens + self.output_tokens
    }
}

/// Streaming event from LLM
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StreamEvent {
    /// Stream started
    MessageStart { message: StreamMessage },
    /// Content block started
    ContentBlockStart {
        index: usize,
        content_block: ContentBlock,
    },
    /// Content block delta (incremental update)
    ContentBlockDelta { index: usize, delta: ContentDelta },
    /// Content block stopped
    ContentBlockStop { index: usize },
    /// Message completed
    MessageDelta {
        delta: MessageDelta,
        usage: TokenUsage,
    },
    /// Stream finished
    MessageStop,
    /// Ping event (keep-alive)
    Ping,
    /// Error event
    Error { error: String },
}

/// Partial message information at stream start
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamMessage {
    pub id: String,
    pub model: String,
    pub role: Role,
    pub usage: TokenUsage,
}

/// Content delta for streaming updates
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentDelta {
    /// Text delta
    TextDelta { text: String },
    /// Tool input delta (JSON)
    InputJsonDelta { partial_json: String },
    /// Thinking delta (Anthropic extended thinking streaming)
    ThinkingDelta { thinking: String },
}

/// Message delta for final updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageDelta {
    pub stop_reason: Option<StopReason>,
    pub stop_sequence: Option<String>,
}

/// Extract all `<think>…</think>` blocks from `text`.
///
/// Returns `(thinking_content, cleaned_text)` where:
/// - `thinking_content` — concatenated inner text of every `<think>` block, trimmed
/// - `cleaned_text`     — original text with all `<think>` blocks removed and trimmed
///
/// Used to separate on-the-fly reasoning traces (DeepSeek-R1, QwQ-32B, etc.)
/// from the visible response text for both streaming and non-streaming paths.
pub fn extract_think_tags(text: &str) -> (String, String) {
    const OPEN: &str = "<think>";
    const CLOSE: &str = "</think>";

    let mut thinking = String::new();
    let mut cleaned = String::new();
    let mut remaining = text;

    while let Some(open_pos) = remaining.find(OPEN) {
        // Text before the opening tag belongs to the visible response.
        cleaned.push_str(&remaining[..open_pos]);
        remaining = &remaining[open_pos + OPEN.len()..];

        if let Some(close_pos) = remaining.find(CLOSE) {
            if !thinking.is_empty() {
                thinking.push('\n');
            }
            thinking.push_str(&remaining[..close_pos]);
            remaining = &remaining[close_pos + CLOSE.len()..];
        } else {
            // No closing tag — treat the rest as thinking (truncated stream).
            if !thinking.is_empty() {
                thinking.push('\n');
            }
            thinking.push_str(remaining);
            remaining = "";
            break;
        }
    }

    cleaned.push_str(remaining);

    (thinking.trim().to_string(), cleaned.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let user_msg = Message::user("Hello");
        assert_eq!(user_msg.role, Role::User);
        assert_eq!(user_msg.content.len(), 1);

        let assistant_msg = Message::assistant("Hi there");
        assert_eq!(assistant_msg.role, Role::Assistant);
    }

    #[test]
    fn test_llm_request_builder() {
        let request = LLMRequest::new("claude-3-sonnet-20240229", vec![Message::user("Test")])
            .with_system("You are helpful")
            .with_temperature(0.7)
            .with_max_tokens(1000)
            .with_streaming();

        assert_eq!(request.model, "claude-3-sonnet-20240229");
        assert!(request.system.is_some());
        assert_eq!(request.temperature, Some(0.7));
        assert_eq!(request.max_tokens, Some(1000));
        assert!(request.stream);
    }

    #[test]
    fn test_token_usage() {
        let usage = TokenUsage {
            input_tokens: 100,
            output_tokens: 200,
        };
        assert_eq!(usage.total(), 300);
    }

    #[test]
    fn with_thinking_sets_temperature_and_config() {
        let req = LLMRequest::new("claude-3-7-sonnet", vec![]).with_thinking(8192);
        assert_eq!(req.temperature, Some(1.0));
        let tc = req.thinking.expect("thinking must be set");
        assert_eq!(tc.r#type, "enabled");
        assert_eq!(tc.budget_tokens, 8192);
    }

    #[test]
    fn with_thinking_sets_temperature() {
        let req = LLMRequest::new("claude-3-7-sonnet", vec![]).with_thinking(8192);
        assert_eq!(req.temperature, Some(1.0));
        let tc = req.thinking.unwrap();
        assert_eq!(tc.r#type, "enabled");
        assert_eq!(tc.budget_tokens, 8192);
    }

    #[test]
    fn with_thinking_zero_budget_is_noop() {
        let req = LLMRequest::new("model", vec![]).with_thinking(0);
        assert!(
            req.thinking.is_none(),
            "zero budget should not enable thinking"
        );
    }

    #[test]
    fn cache_metrics_hit_rate() {
        let cm = CacheMetrics {
            read_tokens: 800,
            creation_tokens: 200,
        };
        assert!((cm.hit_rate() - 0.8).abs() < 0.001);
        let empty = CacheMetrics::default();
        assert_eq!(empty.hit_rate(), 0.0);
    }

    #[test]
    fn perf_metrics_tokens_per_second() {
        let pm = PerfMetrics {
            eval_duration_ms: Some(2_000),
            ..Default::default()
        };
        assert_eq!(pm.tokens_per_second(100), Some(50.0));
    }

    #[test]
    fn perf_metrics_tokens_per_second_missing_duration() {
        let pm = PerfMetrics::default();
        assert_eq!(pm.tokens_per_second(100), None);
    }

    #[test]
    fn perf_metrics_tokens_per_second_zero_duration() {
        let pm = PerfMetrics {
            eval_duration_ms: Some(0),
            ..Default::default()
        };
        assert_eq!(pm.tokens_per_second(100), None);
    }

    #[test]
    fn extract_think_tags_single_block() {
        let (thinking, cleaned) =
            extract_think_tags("<think>I reason here</think>The answer is 42.");
        assert_eq!(thinking, "I reason here");
        assert_eq!(cleaned, "The answer is 42.");
    }

    #[test]
    fn extract_think_tags_multiple_blocks() {
        let (thinking, cleaned) =
            extract_think_tags("<think>first</think> middle <think>second</think> end");
        assert_eq!(thinking, "first\nsecond");
        // " middle " + " end" → two spaces between words after block removal
        assert_eq!(cleaned, "middle  end");
    }

    #[test]
    fn extract_think_tags_no_tags() {
        let (thinking, cleaned) = extract_think_tags("plain text");
        assert!(thinking.is_empty());
        assert_eq!(cleaned, "plain text");
    }

    #[test]
    fn extract_think_tags_unclosed() {
        let (thinking, cleaned) = extract_think_tags("<think>reasoning truncated");
        assert_eq!(thinking, "reasoning truncated");
        assert!(cleaned.is_empty());
    }

    #[test]
    fn extract_think_tags_only_block() {
        let (thinking, cleaned) = extract_think_tags("<think>only thinking</think>");
        assert_eq!(thinking, "only thinking");
        assert!(cleaned.is_empty());
    }
}
