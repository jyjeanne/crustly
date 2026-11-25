---
type: Rust Method
title: new
resource: src/llm/provider/anthropic.rs#L35-L45
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/subagents/ChildTemplate/build
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/anthropic/test_anthropic_provider_creation
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/anthropic/test_supported_models
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/anthropic/test_context_window
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/anthropic/test_cost_calculation
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/anthropic/test_cost_calculation_falls_back_to_family_tier_for_unlisted_model_ids
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/anthropic/test_cost_calculation_unknown_model_family_returns_zero
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/anthropic/test_capabilities
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/factory/create_anthropic
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(api_key: String) -> Self`

# Calls

- [build](../../../../../../functions/src/llm/agent/service/subagents/ChildTemplate/build.md)

# Called by

- [test_anthropic_provider_creation](../../../../../../functions/src/llm/provider/anthropic/test_anthropic_provider_creation.md)
- [test_supported_models](../../../../../../functions/src/llm/provider/anthropic/test_supported_models.md)
- [test_context_window](../../../../../../functions/src/llm/provider/anthropic/test_context_window.md)
- [test_cost_calculation](../../../../../../functions/src/llm/provider/anthropic/test_cost_calculation.md)
- [test_cost_calculation_falls_back_to_family_tier_for_unlisted_model_ids](../../../../../../functions/src/llm/provider/anthropic/test_cost_calculation_falls_back_to_family_tier_for_unlisted_model_ids.md)
- [test_cost_calculation_unknown_model_family_returns_zero](../../../../../../functions/src/llm/provider/anthropic/test_cost_calculation_unknown_model_family_returns_zero.md)
- [test_capabilities](../../../../../../functions/src/llm/provider/anthropic/test_capabilities.md)
- [create_anthropic](../../../../../../functions/src/llm/provider/factory/create_anthropic.md)