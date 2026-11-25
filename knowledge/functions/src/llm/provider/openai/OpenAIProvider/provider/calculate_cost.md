---
type: Rust Method
title: calculate_cost
resource: src/llm/provider/openai.rs#L733-L760
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/openai/test_calculate_cost
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/openai/test_calculate_cost_current_models_are_not_free
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/openai/test_calculate_cost_unknown_model_returns_zero
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn calculate_cost(&self, model: &str, input_tokens: u32, output_tokens: u32) -> f64`

# Called by

- [test_calculate_cost](../../../../../../../functions/src/llm/provider/openai/test_calculate_cost.md)
- [test_calculate_cost_current_models_are_not_free](../../../../../../../functions/src/llm/provider/openai/test_calculate_cost_current_models_are_not_free.md)
- [test_calculate_cost_unknown_model_returns_zero](../../../../../../../functions/src/llm/provider/openai/test_calculate_cost_unknown_model_returns_zero.md)