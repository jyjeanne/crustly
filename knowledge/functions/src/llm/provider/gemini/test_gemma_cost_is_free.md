---
type: Rust Function
title: test_gemma_cost_is_free
resource: src/llm/provider/gemini.rs#L933-L939
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
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

`fn test_gemma_cost_is_free()`

# Calls

- [new](../../../../../functions/src/llm/provider/gemini/GeminiProvider/new.md)
- [calculate_cost](../../../../../functions/src/llm/provider/gemini/GeminiProvider/provider/calculate_cost.md)