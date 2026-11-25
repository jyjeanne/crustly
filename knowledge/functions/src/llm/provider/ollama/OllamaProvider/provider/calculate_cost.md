---
type: Rust Method
title: calculate_cost
resource: src/llm/provider/ollama.rs#L674-L677
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/ollama/test_calculate_cost_is_always_zero
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn calculate_cost(&self, _model: &str, _input_tokens: u32, _output_tokens: u32) -> f64`

# Called by

- [test_calculate_cost_is_always_zero](../../../../../../../functions/src/llm/provider/ollama/test_calculate_cost_is_always_zero.md)