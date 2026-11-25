---
type: Rust Method
title: calculate_cost
resource: src/llm/provider/qwen.rs#L1612-L1640
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/qwen/QwenProvider/is_local
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/qwen/test_calculate_cost_local
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_calculate_cost_cloud
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_calculate_cost_unknown_cloud_model_returns_zero
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn calculate_cost(&self, model: &str, input_tokens: u32, output_tokens: u32) -> f64`

# Calls

- [is_local](../../../../../../../functions/src/llm/provider/qwen/QwenProvider/is_local.md)

# Called by

- [test_calculate_cost_local](../../../../../../../functions/src/llm/provider/qwen/test_calculate_cost_local.md)
- [test_calculate_cost_cloud](../../../../../../../functions/src/llm/provider/qwen/test_calculate_cost_cloud.md)
- [test_calculate_cost_unknown_cloud_model_returns_zero](../../../../../../../functions/src/llm/provider/qwen/test_calculate_cost_unknown_cloud_model_returns_zero.md)