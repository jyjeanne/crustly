---
type: Rust Method
title: calculate_cost
resource: src/llm/provider/qwen.rs#L1586-L1614
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/provider/qwen/QwenProvider/is_local
---

# Signature

`fn calculate_cost(&self, model: &str, input_tokens: u32, output_tokens: u32) -> f64`

# Calls

- [is_local](../../../../../../../functions/src/llm/provider/qwen/QwenProvider/is_local.md)