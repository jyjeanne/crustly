//! Context-window accounting for one request.
//!
//! Every request carries a fixed part - the system prompt and the tool
//! schemas - before any conversation. On a small local window that fixed part
//! can take most of the window, so the agent loop measures it, sizes
//! `max_tokens` from what is left, frees context when too little is left, and
//! reports the numbers to the user.
//!
//! Counts come from tiktoken `cl100k` (see [`token_count`]), which is not the
//! tokenizer of most models. [`calibration_from`] turns the provider's own
//! prompt token count into a correction factor for later estimates.

use super::context::token_count;
use crate::llm::provider::Tool;

/// Most tokens a single answer may use.
pub const MAX_COMPLETION_TOKENS: u32 = 4096;

/// Allowance for the chat-template and framing tokens providers add around
/// the prompt (Qwen's Hermes adapter also embeds the tool schemas in its
/// system text), which the estimate cannot see.
pub const PROMPT_FRAMING_MARGIN: u32 = 1024;

/// Window assumed when the provider does not report one.
pub const DEFAULT_CONTEXT_WINDOW: u32 = 4096;

/// Upper bound on the calibration factor, so one odd response cannot make
/// every later estimate absurdly conservative.
const MAX_CALIBRATION: f64 = 2.0;

/// Token cost of one request against the model's context window.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ContextBudget {
    /// The model's context window.
    pub window: u32,
    /// Estimated tokens of the system prompt (including any compaction summary).
    pub system_prompt_tokens: u32,
    /// Estimated tokens of the serialized tool schemas.
    pub tool_schema_tokens: u32,
    /// Number of tool schemas sent.
    pub tool_count: u32,
    /// Estimated tokens of the conversation messages.
    pub history_tokens: u32,
    /// Provider prompt tokens per estimated token, never below 1.0.
    pub calibration: f64,
}

impl ContextBudget {
    /// Measure a request from its parts.
    pub fn measure(
        window: u32,
        system_prompt: Option<&str>,
        tools: &[Tool],
        history_tokens: usize,
        calibration: f64,
    ) -> Self {
        Self::from_counts(
            window,
            system_prompt.map(token_count).unwrap_or(0),
            tool_schema_tokens(tools),
            tools.len() as u32,
            history_tokens,
            calibration,
        )
    }

    /// Build a budget from counts already taken (the tool schemas do not
    /// change within a request, so callers count them once).
    pub fn from_counts(
        window: u32,
        system_prompt_tokens: u32,
        tool_schema_tokens: u32,
        tool_count: u32,
        history_tokens: usize,
        calibration: f64,
    ) -> Self {
        Self {
            window,
            system_prompt_tokens,
            tool_schema_tokens,
            tool_count,
            history_tokens: u32::try_from(history_tokens).unwrap_or(u32::MAX),
            calibration: calibration.clamp(1.0, MAX_CALIBRATION),
        }
    }

    /// The uncalibrated prompt estimate: system prompt, tools and history.
    pub fn raw_prompt_tokens(&self) -> u32 {
        self.system_prompt_tokens
            .saturating_add(self.tool_schema_tokens)
            .saturating_add(self.history_tokens)
    }

    fn calibrated(&self, tokens: u32) -> u32 {
        (tokens as f64 * self.calibration).ceil() as u32
    }

    /// Estimated prompt size including provider framing, before clamping it to
    /// the context window. Callers use this to distinguish a full window from
    /// an estimate that already exceeds it.
    pub fn estimated_prompt_tokens(&self) -> u32 {
        self.calibrated(self.raw_prompt_tokens())
            .saturating_add(PROMPT_FRAMING_MARGIN)
    }

    /// What every request costs before any conversation: system prompt, tool
    /// schemas and the framing margin.
    pub fn fixed_overhead(&self) -> u32 {
        self.calibrated(
            self.system_prompt_tokens
                .saturating_add(self.tool_schema_tokens),
        )
        .saturating_add(PROMPT_FRAMING_MARGIN)
    }

    /// Tokens the prompt is expected to take, framing margin included.
    pub fn used(&self) -> u32 {
        self.estimated_prompt_tokens().min(self.window)
    }

    /// Tokens left in the window for the answer.
    pub fn available(&self) -> u32 {
        self.window.saturating_sub(self.used())
    }

    /// The `max_tokens` to send: what is left, capped at
    /// [`MAX_COMPLETION_TOKENS`] and never zero (providers reject zero).
    pub fn completion_budget(&self) -> u32 {
        self.available().clamp(1, MAX_COMPLETION_TOKENS)
    }

    /// The smallest answer budget worth sending a request with: an eighth of
    /// the window, between 256 and 1,024 tokens.
    pub fn min_answer_tokens(&self) -> u32 {
        (self.window / 8).clamp(256, 1024)
    }

    /// True when less than [`Self::min_answer_tokens`] is left for the answer.
    pub fn is_tight(&self) -> bool {
        self.available() < self.min_answer_tokens()
    }

    /// The numbers shown to the user.
    pub fn usage(&self, core_tools_only: bool) -> ContextUsage {
        ContextUsage {
            window: self.window,
            used: self.used(),
            fixed_overhead: self.fixed_overhead(),
            available: self.available(),
            tool_count: self.tool_count,
            core_tools_only,
        }
    }
}

/// Context-window figures for display: the TUI header, Model Info panel and
/// `crustly run` output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContextUsage {
    /// The model's context window.
    pub window: u32,
    /// Tokens the last request's prompt took, framing margin included.
    pub used: u32,
    /// System prompt + tool schemas + framing margin.
    pub fixed_overhead: u32,
    /// Tokens that were left for the answer.
    pub available: u32,
    /// Number of tool schemas sent.
    pub tool_count: u32,
    /// Whether only the core tool set was exposed.
    pub core_tools_only: bool,
}

impl ContextUsage {
    /// True when less than an eighth of the window (at most 1,024 tokens) is
    /// left once the fixed overhead is paid - the point at which a local model
    /// can barely answer, let alone read a file.
    pub fn overhead_is_tight(&self) -> bool {
        let floor = (self.window / 8).clamp(256, 1024);
        self.window.saturating_sub(self.fixed_overhead) < floor
    }
}

/// Tokens of the tool schemas as sent to the provider.
pub fn tool_schema_tokens(tools: &[Tool]) -> u32 {
    if tools.is_empty() {
        return 0;
    }
    serde_json::to_string(tools)
        .map(|serialized| token_count(&serialized))
        .unwrap_or(0)
}

/// Calibration factor from the provider's own prompt token count.
///
/// Returns `None` when either count is unknown or when the provider counted
/// fewer tokens than estimated. Only corrections upward are kept: a provider
/// that reuses a cached prompt prefix (Ollama) may report fewer prompt tokens
/// than it holds in context, and trusting that would under-reserve.
pub fn calibration_from(provider_prompt_tokens: u32, estimated_prompt_tokens: u32) -> Option<f64> {
    if provider_prompt_tokens == 0 || estimated_prompt_tokens == 0 {
        return None;
    }
    let ratio = provider_prompt_tokens as f64 / estimated_prompt_tokens as f64;
    (ratio > 1.0).then(|| ratio.min(MAX_CALIBRATION))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn budget(window: u32, system: u32, tools: u32, history: usize) -> ContextBudget {
        ContextBudget::from_counts(window, system, tools, 23, history, 1.0)
    }

    /// The measured default for Crustly's full tool set on master: a 1,784
    /// token system prompt and 4,799 tokens of tool schemas leave 585 tokens
    /// of an 8,192 window.
    #[test]
    fn full_tool_set_leaves_little_of_an_8k_window() {
        let b = budget(8192, 1784, 4799, 0);
        assert_eq!(b.fixed_overhead(), 7607);
        assert_eq!(b.available(), 585);
        assert!(b.is_tight());
        assert!(b.usage(false).overhead_is_tight());
    }

    #[test]
    fn completion_budget_is_never_zero_and_never_above_the_cap() {
        assert_eq!(budget(8192, 4000, 4000, 4000).completion_budget(), 1);
        assert_eq!(
            budget(200_000, 1000, 1000, 0).completion_budget(),
            MAX_COMPLETION_TOKENS
        );
    }

    #[test]
    fn distinguishes_prompt_overflow_from_a_clamped_full_window() {
        let b = budget(8192, 6000, 1000, 1500);
        assert_eq!(b.available(), 0);
        assert!(b.estimated_prompt_tokens() > b.window);
    }

    #[test]
    fn calibration_scales_the_estimate_up() {
        let plain = budget(32_768, 1000, 1000, 1000);
        let calibrated = ContextBudget::from_counts(32_768, 1000, 1000, 23, 1000, 1.5);
        assert_eq!(calibrated.used(), 3000 * 3 / 2 + PROMPT_FRAMING_MARGIN);
        assert!(calibrated.available() < plain.available());
    }

    #[test]
    fn calibration_never_goes_below_one() {
        let b = ContextBudget::from_counts(8192, 100, 100, 1, 100, 0.5);
        assert_eq!(b.calibration, 1.0);
    }

    #[test]
    fn calibration_from_keeps_upward_corrections_only() {
        assert_eq!(calibration_from(1200, 1000), Some(1.2));
        assert_eq!(calibration_from(800, 1000), None);
        assert_eq!(calibration_from(0, 1000), None);
        assert_eq!(calibration_from(1000, 0), None);
        assert_eq!(calibration_from(5000, 1000), Some(MAX_CALIBRATION));
    }

    #[test]
    fn min_answer_tokens_scales_with_the_window() {
        assert_eq!(budget(2048, 0, 0, 0).min_answer_tokens(), 256);
        assert_eq!(budget(4096, 0, 0, 0).min_answer_tokens(), 512);
        assert_eq!(budget(200_000, 0, 0, 0).min_answer_tokens(), 1024);
    }
}
