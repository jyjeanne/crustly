---
type: Rust Function
title: test_calculate_cost_gemini_flash
resource: src/llm/provider/gemini.rs#L942-L947
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

`fn test_calculate_cost_gemini_flash()`

# Calls

- [new](../../../../../functions/src/llm/provider/gemini/GeminiProvider/new.md)
- [calculate_cost](../../../../../functions/src/llm/provider/gemini/GeminiProvider/provider/calculate_cost.md)