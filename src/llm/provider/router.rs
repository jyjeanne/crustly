//! Model tier routing: maps request complexity to a (provider, model, token limits) triple.

use serde::{Deserialize, Serialize};

/// Complexity tier for a request. Selected by `prompt_analyzer`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ModelTier {
    /// Single-turn lookups, completions, simple questions.
    Fast,
    /// Multi-step edits, explanations, moderate analysis.
    Balanced,
    /// Architectural refactors, deep debugging, cross-file analysis.
    Powerful,
}

/// Routes a `ModelTier` to a concrete provider + model combination.
#[derive(Debug, Clone)]
pub struct ModelRouter {
    fast: (String, String),
    balanced: (String, String),
    powerful: (String, String),
}

impl ModelRouter {
    pub fn new(
        fast_provider: &str,
        fast_model: &str,
        balanced_provider: &str,
        balanced_model: &str,
        powerful_provider: &str,
        powerful_model: &str,
    ) -> Self {
        Self {
            fast: (fast_provider.to_string(), fast_model.to_string()),
            balanced: (balanced_provider.to_string(), balanced_model.to_string()),
            powerful: (powerful_provider.to_string(), powerful_model.to_string()),
        }
    }

    /// Resolve a tier to (provider_name, model_id).
    pub fn resolve(&self, tier: ModelTier) -> (&str, &str) {
        match tier {
            ModelTier::Fast => (&self.fast.0, &self.fast.1),
            ModelTier::Balanced => (&self.balanced.0, &self.balanced.1),
            ModelTier::Powerful => (&self.powerful.0, &self.powerful.1),
        }
    }

    /// Maximum output tokens for a tier.
    pub fn max_output_tokens(&self, tier: ModelTier) -> u32 {
        match tier {
            ModelTier::Fast => 2_048,
            ModelTier::Balanced => 8_192,
            ModelTier::Powerful => 16_384,
        }
    }

    /// Provider context window (input limit) for a tier.
    pub fn context_window(&self, tier: ModelTier) -> u32 {
        match tier {
            ModelTier::Fast => 100_000,
            ModelTier::Balanced | ModelTier::Powerful => 200_000,
        }
    }

    /// Thinking token budget for a tier (`None` = disabled).
    pub fn thinking_budget(&self, tier: ModelTier) -> Option<u32> {
        match tier {
            ModelTier::Fast => None,
            ModelTier::Balanced => Some(8_192),
            ModelTier::Powerful => Some(16_384),
        }
    }

    /// Default router using Anthropic claude models.
    pub fn default_anthropic() -> Self {
        Self::new(
            "anthropic", "claude-haiku-4-5-20251001",
            "anthropic", "claude-sonnet-4-6",
            "anthropic", "claude-opus-4-7",
        )
    }

    /// Test-only router using the same model for all tiers (avoids network calls).
    #[cfg(test)]
    pub fn default_for_test() -> Self {
        Self::new(
            "anthropic", "claude-haiku-4-5-20251001",
            "anthropic", "claude-sonnet-4-6",
            "anthropic", "claude-opus-4-7",
        )
    }
}

impl Default for ModelRouter {
    fn default() -> Self {
        Self::default_anthropic()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_tiers_resolve_to_non_empty_model() {
        let router = ModelRouter::default_for_test();
        for tier in [ModelTier::Fast, ModelTier::Balanced, ModelTier::Powerful] {
            let (provider, model) = router.resolve(tier);
            assert!(!provider.is_empty(), "{:?} tier must have a provider", tier);
            assert!(!model.is_empty(), "{:?} tier must have a model_id", tier);
        }
    }

    #[test]
    fn token_limits_ordered_correctly() {
        let router = ModelRouter::default_for_test();
        assert!(router.max_output_tokens(ModelTier::Fast) < router.max_output_tokens(ModelTier::Balanced));
        assert!(router.max_output_tokens(ModelTier::Balanced) < router.max_output_tokens(ModelTier::Powerful));
        assert!(router.max_output_tokens(ModelTier::Fast) >= 2_048);
        assert!(router.max_output_tokens(ModelTier::Powerful) >= 16_384);
    }

    #[test]
    fn thinking_budget_by_tier() {
        let router = ModelRouter::default_for_test();
        assert!(router.thinking_budget(ModelTier::Fast).is_none());
        assert_eq!(router.thinking_budget(ModelTier::Balanced), Some(8_192));
        assert_eq!(router.thinking_budget(ModelTier::Powerful), Some(16_384));
    }
}
