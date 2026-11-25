---
type: Rust Function
title: test_calculate_cost_unknown_cloud_model_returns_zero
resource: src/llm/provider/qwen.rs#L2394-L2397
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/qwen/QwenProvider/dashscope_intl
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/provider/calculate_cost
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn test_calculate_cost_unknown_cloud_model_returns_zero()`

# Calls

- [dashscope_intl](../../../../../functions/src/llm/provider/qwen/QwenProvider/dashscope_intl.md)
- [calculate_cost](../../../../../functions/src/llm/provider/qwen/QwenProvider/provider/calculate_cost.md)