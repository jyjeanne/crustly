---
type: Rust Function
title: test_calculate_cost_unknown_model_returns_zero
resource: src/llm/provider/openai.rs#L1426-L1432
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/openai/OpenAIProvider/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/openai/OpenAIProvider/provider/calculate_cost
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn test_calculate_cost_unknown_model_returns_zero()`

# Calls

- [new](../../../../../functions/src/llm/provider/openai/OpenAIProvider/new.md)
- [calculate_cost](../../../../../functions/src/llm/provider/openai/OpenAIProvider/provider/calculate_cost.md)