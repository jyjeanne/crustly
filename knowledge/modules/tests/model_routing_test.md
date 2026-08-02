---
type: Rust Module
title: model_routing_test
resource: tests/model_routing_test.rs#L1-L102
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/crustly-llm-provider-router-modelrouter-modeltier
  - external/crustly-llm-provider-types-llmrequest
  - external/crustly-tui-prompt-analyzer-promptanalyzer
  member_of:
  - packages/crustly
---

# Contains

- [all_tiers_resolve_to_valid_model](../../functions/tests/model_routing_test/all_tiers_resolve_to_valid_model.md)
- [thinking_config_forces_temperature_one](../../functions/tests/model_routing_test/thinking_config_forces_temperature_one.md)
- [zero_budget_does_not_enable_thinking](../../functions/tests/model_routing_test/zero_budget_does_not_enable_thinking.md)
- [token_limits_ordered_by_tier](../../functions/tests/model_routing_test/token_limits_ordered_by_tier.md)
- [complex_prompt_routes_to_powerful_tier](../../functions/tests/model_routing_test/complex_prompt_routes_to_powerful_tier.md)
- [simple_prompt_routes_to_fast_tier](../../functions/tests/model_routing_test/simple_prompt_routes_to_fast_tier.md)
- [neutral_prompt_routes_to_balanced_tier](../../functions/tests/model_routing_test/neutral_prompt_routes_to_balanced_tier.md)

# Imports

- `crustly::llm::provider::router::{ModelRouter, ModelTier}`
- `crustly::llm::provider::types::LLMRequest`
- `crustly::tui::prompt_analyzer::PromptAnalyzer`

# Member of

- [crustly](../../packages/crustly.md)