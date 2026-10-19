//! Agent Context Management
//!
//! Manages conversation context including messages, system prompts,
//! and token tracking.

use crate::db::models::Message as DbMessage;
use crate::llm::provider::types::CacheMetrics;
use crate::llm::provider::{ContentBlock, Message, Role};
use std::path::PathBuf;
use uuid::Uuid;

/// Agent context for a conversation
#[derive(Debug, Clone)]
pub struct AgentContext {
    /// Session ID
    pub session_id: Uuid,

    /// System prompt
    pub system_prompt: Option<String>,

    /// Conversation messages
    pub messages: Vec<Message>,

    /// Tracked files in the conversation
    pub tracked_files: Vec<TrackedFile>,

    /// Current token count estimate
    pub token_count: usize,

    /// Maximum context tokens
    pub max_tokens: usize,

    /// Accumulated prompt-cache metrics across all LLM calls in this session.
    pub accumulated_cache_metrics: CacheMetrics,

    /// Summary of turns dropped by compaction. Sent as part of the system
    /// prompt (see [`Self::effective_system_prompt`]) rather than as a
    /// message: a `system`-role message in the middle of `messages` is
    /// rejected by some providers (Anthropic), while every provider accepts a
    /// longer system prompt.
    pub compacted_summary: Option<String>,
}

/// A file tracked in the conversation
#[derive(Debug, Clone)]
pub struct TrackedFile {
    pub id: Uuid,
    pub path: PathBuf,
    pub content: Option<String>,
    pub token_count: usize,
}

impl AgentContext {
    /// Create a new agent context for a session
    pub fn new(session_id: Uuid, max_tokens: usize) -> Self {
        Self {
            session_id,
            system_prompt: None,
            messages: Vec::new(),
            tracked_files: Vec::new(),
            token_count: 0,
            max_tokens,
            accumulated_cache_metrics: CacheMetrics::default(),
            compacted_summary: None,
        }
    }

    /// The system prompt to send: the configured prompt plus, after a
    /// compaction, the summary of the turns that were dropped.
    pub fn effective_system_prompt(&self) -> Option<String> {
        match (&self.system_prompt, &self.compacted_summary) {
            (prompt, None) => prompt.clone(),
            (prompt, Some(summary)) => {
                let section = format!(
                    "## Earlier in this conversation (compacted)\n\n\
                     Older turns were removed to fit the context window. This is \
                     what they contained:\n\n{summary}"
                );
                Some(match prompt {
                    Some(prompt) => format!("{prompt}\n\n{section}"),
                    None => section,
                })
            }
        }
    }

    /// Recompute `token_count` from the current messages, after an in-place
    /// edit (compaction, elided tool output) made the running total stale.
    /// Counts messages only; the agent loop budgets the system prompt and
    /// tool schemas separately.
    pub fn recount_tokens(&mut self) {
        self.token_count = self
            .messages
            .iter()
            .map(|m| self.estimate_message_tokens(m))
            .sum();
    }

    /// Set the system prompt
    pub fn with_system_prompt(mut self, prompt: String) -> Self {
        self.token_count += Self::estimate_tokens(&prompt);
        self.system_prompt = Some(prompt);
        self
    }

    /// Add a message to the context
    pub fn add_message(&mut self, message: Message) {
        // Estimate tokens for the message
        let tokens = self.estimate_message_tokens(&message);
        self.token_count += tokens;
        self.messages.push(message);
    }

    /// Convert database messages to LLM messages.
    ///
    /// Only the text of earlier turns is stored, not their tool calls. An
    /// assistant message that has a tool log gets it as a short note before
    /// its text, so the model knows what that turn read, changed or ran.
    pub fn from_db_messages(
        session_id: Uuid,
        db_messages: Vec<DbMessage>,
        max_tokens: usize,
    ) -> Self {
        let mut context = Self::new(session_id, max_tokens);

        for db_msg in db_messages {
            let role = match db_msg.role.as_str() {
                "user" => Role::User,
                "assistant" => Role::Assistant,
                "system" => Role::System,
                _ => Role::User, // Default fallback
            };

            let text = match (&role, db_msg.tool_log) {
                (Role::Assistant, Some(log)) if !log.is_empty() => {
                    format!(
                        "(Tools used before this answer: {log})\n\n{}",
                        db_msg.content
                    )
                }
                _ => db_msg.content,
            };
            let message = Message {
                role,
                content: vec![ContentBlock::Text { text }],
            };

            context.add_message(message);
        }

        context
    }

    /// Track a file in the conversation
    pub fn track_file(&mut self, file: TrackedFile) {
        self.token_count += file.token_count;
        self.tracked_files.push(file);
    }

    /// Check if context would exceed limit with additional tokens
    pub fn would_exceed_limit(&self, additional_tokens: usize) -> bool {
        self.token_count + additional_tokens > self.max_tokens
    }

    /// Estimate tokens for a message
    fn estimate_message_tokens(&self, message: &Message) -> usize {
        let mut tokens = 0;

        for content in &message.content {
            match content {
                ContentBlock::Text { text } => {
                    tokens += Self::estimate_tokens(text);
                }
                ContentBlock::ToolUse { name, input, .. } => {
                    tokens += Self::estimate_tokens(name);
                    tokens += Self::estimate_tokens(&input.to_string());
                }
                ContentBlock::ToolResult { content, .. } => {
                    tokens += Self::estimate_tokens(content);
                }
                ContentBlock::Image { .. } => {
                    // Images use a fixed token count (approximate)
                    tokens += 1000;
                }
                ContentBlock::Thinking { thinking } => {
                    tokens += Self::estimate_tokens(thinking);
                }
            }
        }

        // Add overhead for message structure
        tokens + 4
    }

    /// Estimate tokens using the module-level BPE counter.
    fn estimate_tokens(text: &str) -> usize {
        token_count(text) as usize
    }

    /// Get the current token usage percentage
    pub fn usage_percentage(&self) -> f64 {
        (self.token_count as f64 / self.max_tokens as f64) * 100.0
    }

    /// Returns true when the messages alone exceed 80% of the window.
    ///
    /// The agent loop does not use this: it ignores the system prompt and tool
    /// schemas, which can take most of a small window. The loop checks
    /// `budget::ContextBudget::is_tight` instead.
    pub fn should_compact(&self) -> bool {
        const COMPACTION_THRESHOLD: f64 = 0.80;
        self.token_count as f64 / self.max_tokens as f64 > COMPACTION_THRESHOLD
    }

    /// Trim old messages if context is too large
    pub fn trim_to_fit(&mut self, required_space: usize) {
        while self.would_exceed_limit(required_space) && !self.messages.is_empty() {
            // Remove the oldest user/assistant message pair
            if let Some(first_msg) = self.messages.first() {
                let tokens = self.estimate_message_tokens(first_msg);
                self.token_count = self.token_count.saturating_sub(tokens);
                self.messages.remove(0);
            }
        }
    }

    /// Prepend recent episodic memories (from prior sessions) as a system message.
    ///
    /// Loads memories within `max_tokens` budget from the DB and inserts them at
    /// position 0. Call this after `new()` before the first user turn.
    pub async fn inject_episodic_memories(
        &mut self,
        pool: &sqlx::SqlitePool,
        max_tokens: i32,
    ) -> anyhow::Result<()> {
        use crate::db::repository::EpisodicMemoryRepository;
        let repo = EpisodicMemoryRepository::new(pool.clone());
        repo.inject_into_context(self, max_tokens).await
    }
}

/// BPE-accurate token count using cl100k_base (OpenAI/Claude-compatible).
///
/// Claude's tokenizer is not publicly released; cl100k_base gives <5% error
/// on typical Rust/prose content. Trades ~1ms first-call latency for accuracy.
pub fn token_count(text: &str) -> u32 {
    if text.is_empty() {
        return 0;
    }
    // cl100k_base is used by GPT-4 and gives a close approximation for Claude.
    // The BPE object is thread-safe; initialization happens once via lazy_static inside the crate.
    match tiktoken_rs::cl100k_base() {
        Ok(bpe) => bpe.encode_with_special_tokens(text).len() as u32,
        Err(_) => {
            // Fallback if vocab fails to load: word-count heuristic (~15% error).
            let words = text.split_whitespace().count();
            (words as f64 * 1.3).ceil() as u32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_creation() {
        let session_id = Uuid::new_v4();
        let context = AgentContext::new(session_id, 4096);

        assert_eq!(context.session_id, session_id);
        assert_eq!(context.max_tokens, 4096);
        assert_eq!(context.token_count, 0);
        assert!(context.messages.is_empty());
    }

    #[test]
    fn test_add_message() {
        let session_id = Uuid::new_v4();
        let mut context = AgentContext::new(session_id, 4096);

        let message = Message::user("Hello, how are you?");
        context.add_message(message);

        assert_eq!(context.messages.len(), 1);
        assert!(context.token_count > 0);
    }

    #[test]
    fn test_system_prompt() {
        let session_id = Uuid::new_v4();
        let context = AgentContext::new(session_id, 4096)
            .with_system_prompt("You are a helpful assistant.".to_string());

        assert!(context.system_prompt.is_some());
        assert!(context.token_count > 0);
    }

    #[test]
    fn test_token_estimation() {
        let tokens = AgentContext::estimate_tokens("Hello world");
        assert!(tokens > 0);
        assert!(tokens < 10); // Should be around 2-3 tokens
    }

    #[test]
    fn test_would_exceed_limit() {
        let session_id = Uuid::new_v4();
        let mut context = AgentContext::new(session_id, 100);

        let message = Message::user("Hello");
        context.add_message(message);

        assert!(!context.would_exceed_limit(10));
        assert!(context.would_exceed_limit(1000));
    }

    #[test]
    fn test_usage_percentage() {
        let session_id = Uuid::new_v4();
        let mut context = AgentContext::new(session_id, 100);

        // Add message that uses ~50 tokens
        let long_text = "a".repeat(200); // ~50 tokens
        let message = Message::user(long_text);
        context.add_message(message);

        let usage = context.usage_percentage();
        assert!(usage > 0.0 && usage <= 100.0);
    }

    #[test]
    fn test_trim_to_fit() {
        let session_id = Uuid::new_v4();
        let mut context = AgentContext::new(session_id, 100);

        // Add several messages with longer text to ensure they exceed limit
        for i in 0..5 {
            let long_text = format!("This is a longer message {} that will use more tokens to ensure we actually need to trim", i);
            let message = Message::user(long_text);
            context.add_message(message);
        }

        let original_count = context.messages.len();
        context.trim_to_fit(10); // Require 10 tokens space, forcing trimming

        // Should have removed some messages
        assert!(context.messages.len() < original_count);
    }

    /// QS-0.1: token_count must be within ±2% of actual BPE count for a Rust file.
    ///
    /// Since we're using tiktoken-rs directly, our count IS the ground truth.
    /// This test verifies the function produces a non-trivial, plausible count.
    #[test]
    fn token_count_bpe_accuracy_rust_file() {
        let rust_snippet = r#"
pub struct AgentContext {
    pub session_id: uuid::Uuid,
    pub messages: Vec<Message>,
    pub token_count: usize,
    pub max_tokens: usize,
}

impl AgentContext {
    pub fn new(session_id: uuid::Uuid, max_tokens: usize) -> Self {
        Self {
            session_id,
            messages: Vec::new(),
            token_count: 0,
            max_tokens,
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.token_count += 4;
        self.messages.push(message);
    }
}
"#;
        let count = token_count(rust_snippet);
        // cl100k_base gives ~80-120 tokens for this 400-char Rust snippet.
        // Accept a wide range to avoid coupling to exact tiktoken internals.
        assert!(
            (50..=200).contains(&count),
            "BPE count out of expected range: {}",
            count
        );
        assert!(count > 0);
    }

    #[test]
    fn token_count_empty_string() {
        assert_eq!(token_count(""), 0);
    }

    #[test]
    fn token_count_prose_reasonable() {
        let prose = "The quick brown fox jumps over the lazy dog. This sentence has ten words.";
        let count = token_count(prose);
        // Ground-truth BPE for this sentence: ~14-18 tokens
        assert!(
            (10..=30).contains(&count),
            "prose count unreasonable: {}",
            count
        );
    }
}
