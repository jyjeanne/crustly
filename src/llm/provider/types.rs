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
        if total == 0 { 0.0 } else { self.read_tokens as f32 / total as f32 }
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
}

/// Message delta for final updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageDelta {
    pub stop_reason: Option<StopReason>,
    pub stop_sequence: Option<String>,
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
        let req = LLMRequest::new("claude-3-7-sonnet", vec![])
            .with_thinking(8192);
        assert_eq!(req.temperature, Some(1.0));
        let tc = req.thinking.unwrap();
        assert_eq!(tc.r#type, "enabled");
        assert_eq!(tc.budget_tokens, 8192);
    }

    #[test]
    fn with_thinking_zero_budget_is_noop() {
        let req = LLMRequest::new("model", vec![]).with_thinking(0);
        assert!(req.thinking.is_none(), "zero budget should not enable thinking");
    }

    #[test]
    fn cache_metrics_hit_rate() {
        let cm = CacheMetrics { read_tokens: 800, creation_tokens: 200 };
        assert!((cm.hit_rate() - 0.8).abs() < 0.001);
        let empty = CacheMetrics::default();
        assert_eq!(empty.hit_rate(), 0.0);
    }
}
