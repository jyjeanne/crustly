---
type: Rust Function
title: test_calculate_cost_current_models_are_not_free
resource: src/llm/provider/openai.rs#L1415-L1423
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

`fn test_calculate_cost_current_models_are_not_free()`

# Calls

- [new](../../../../../functions/src/llm/provider/openai/OpenAIProvider/new.md)
- [calculate_cost](../../../../../functions/src/llm/provider/openai/OpenAIProvider/provider/calculate_cost.md)