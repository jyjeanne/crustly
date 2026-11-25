---
type: Rust Method
title: calculate_cost
resource: src/llm/provider/gemini.rs#L704-L721
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/gemini/test_calculate_cost_gemini_flash
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/gemini/test_gemma_cost_is_free
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/gemini/test_calculate_cost_all_known_models
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn calculate_cost(&self, model: &str, input_tokens: u32, output_tokens: u32) -> f64`

# Called by

- [test_calculate_cost_gemini_flash](../../../../../../../functions/src/llm/provider/gemini/test_calculate_cost_gemini_flash.md)
- [test_gemma_cost_is_free](../../../../../../../functions/src/llm/provider/gemini/test_gemma_cost_is_free.md)
- [test_calculate_cost_all_known_models](../../../../../../../functions/src/llm/provider/gemini/test_calculate_cost_all_known_models.md)