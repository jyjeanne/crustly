---
type: Rust Function
title: test_calculate_cost_all_known_models
resource: src/llm/provider/gemini.rs#L1203-L1217
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/gemini/GeminiProvider/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/gemini/GeminiProvider/provider/calculate_cost
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn test_calculate_cost_all_known_models()`

# Calls

- [new](../../../../../functions/src/llm/provider/gemini/GeminiProvider/new.md)
- [calculate_cost](../../../../../functions/src/llm/provider/gemini/GeminiProvider/provider/calculate_cost.md)