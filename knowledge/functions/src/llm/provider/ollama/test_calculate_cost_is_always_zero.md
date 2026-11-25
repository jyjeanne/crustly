---
type: Rust Function
title: test_calculate_cost_is_always_zero
resource: src/llm/provider/ollama.rs#L1115-L1118
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/ollama/OllamaProvider/default_local
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaProvider/provider/calculate_cost
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn test_calculate_cost_is_always_zero()`

# Calls

- [default_local](../../../../../functions/src/llm/provider/ollama/OllamaProvider/default_local.md)
- [calculate_cost](../../../../../functions/src/llm/provider/ollama/OllamaProvider/provider/calculate_cost.md)