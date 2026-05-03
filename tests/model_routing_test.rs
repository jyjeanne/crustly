//! Tests for ModelRouter, ThinkingConfig, and prompt complexity classification
//! (QS-2.2, QS-5, FR-004, FR-005, Contract 5).

use crustly::llm::provider::router::{ModelRouter, ModelTier};
use crustly::llm::provider::types::LLMRequest;
use crustly::tui::prompt_analyzer::PromptAnalyzer;

/// Contract 5 / QS-5: All three tiers resolve to non-empty (provider, model) pairs.
#[test]
fn all_tiers_resolve_to_valid_model() {
    let router = ModelRouter::default();
    for tier in [ModelTier::Fast, ModelTier::Balanced, ModelTier::Powerful] {
        let (provider, model) = router.resolve(tier);
        assert!(!provider.is_empty(), "{:?} must have a provider", tier);
        assert!(!model.is_empty(), "{:?} must have a model_id", tier);
    }
}

/// QS-2.2: with_thinking() forces temperature to 1.0.
#[test]
fn thinking_config_forces_temperature_one() {
    let req = LLMRequest::new("model", vec![]).with_thinking(8192);
    assert_eq!(req.temperature, Some(1.0));
    let thinking = req.thinking.expect("thinking must be Some");
    assert_eq!(thinking.budget_tokens, 8192);
    assert_eq!(thinking.r#type, "enabled");
}

/// with_thinking(0) must NOT enable thinking.
#[test]
fn zero_budget_does_not_enable_thinking() {
    let req = LLMRequest::new("model", vec![]).with_thinking(0);
    assert!(req.thinking.is_none(), "zero budget must leave thinking disabled");
}

/// Contract 5: Powerful tier must have a higher max_output_tokens than Fast.
#[test]
fn token_limits_ordered_by_tier() {
    let router = ModelRouter::default();
    assert!(
        router.max_output_tokens(ModelTier::Fast) < router.max_output_tokens(ModelTier::Balanced),
        "Balanced must have more output tokens than Fast"
    );
    assert!(
        router.max_output_tokens(ModelTier::Balanced)
            < router.max_output_tokens(ModelTier::Powerful),
        "Powerful must have more output tokens than Balanced"
    );
}

/// FR-004: Architectural/debugging prompts → Powerful.
#[test]
fn complex_prompt_routes_to_powerful_tier() {
    let analyzer = PromptAnalyzer::new();
    for prompt in &[
        "refactor the auth module to use repository pattern",
        "debug this race condition in the async handler",
        "analyze the architecture of the database layer",
        "which files would be affected by this interface change",
    ] {
        let tier = analyzer.classify_tier(prompt);
        assert_eq!(
            tier,
            ModelTier::Powerful,
            "prompt '{}' must classify as Powerful, got {:?}",
            prompt,
            tier
        );
    }
}

/// FR-004: Simple questions → Fast.
#[test]
fn simple_prompt_routes_to_fast_tier() {
    let analyzer = PromptAnalyzer::new();
    for prompt in &[
        "what is the current file?",
        "summarize this text",
        "list the dependencies",
    ] {
        let tier = analyzer.classify_tier(prompt);
        assert_eq!(
            tier,
            ModelTier::Fast,
            "prompt '{}' must classify as Fast, got {:?}",
            prompt,
            tier
        );
    }
}

/// FR-004: Default (no strong signals) → Balanced.
#[test]
fn neutral_prompt_routes_to_balanced_tier() {
    let analyzer = PromptAnalyzer::new();
    let prompt = "implement the login feature";
    let tier = analyzer.classify_tier(prompt);
    assert_eq!(tier, ModelTier::Balanced);
}
