---
type: Rust Function
title: test_custom_default_model
resource: src/llm/provider/gemini.rs#L909-L913
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
  - target: functions/src/llm/provider/gemini/GeminiProvider/with_default_model
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/gemini/GeminiProvider/provider/default_model
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn test_custom_default_model()`

# Calls

- [new](../../../../../functions/src/llm/provider/gemini/GeminiProvider/new.md)
- [with_default_model](../../../../../functions/src/llm/provider/gemini/GeminiProvider/with_default_model.md)
- [default_model](../../../../../functions/src/llm/provider/gemini/GeminiProvider/provider/default_model.md)