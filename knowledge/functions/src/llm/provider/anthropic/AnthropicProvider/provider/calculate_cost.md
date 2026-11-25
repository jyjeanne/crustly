---
type: Rust Method
title: calculate_cost
resource: src/llm/provider/anthropic.rs#L333-L372
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/anthropic/test_cost_calculation
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/anthropic/test_cost_calculation_falls_back_to_family_tier_for_unlisted_model_ids
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/anthropic/test_cost_calculation_unknown_model_family_returns_zero
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn calculate_cost(&self, model: &str, input_tokens: u32, output_tokens: u32) -> f64`

# Called by

- [test_cost_calculation](../../../../../../../functions/src/llm/provider/anthropic/test_cost_calculation.md)
- [test_cost_calculation_falls_back_to_family_tier_for_unlisted_model_ids](../../../../../../../functions/src/llm/provider/anthropic/test_cost_calculation_falls_back_to_family_tier_for_unlisted_model_ids.md)
- [test_cost_calculation_unknown_model_family_returns_zero](../../../../../../../functions/src/llm/provider/anthropic/test_cost_calculation_unknown_model_family_returns_zero.md)