---
type: Rust Function
title: test_gemini_provider_creation
resource: src/llm/provider/gemini.rs#L901-L906
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
  - target: functions/src/llm/provider/gemini/GeminiProvider/provider/name
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/gemini/GeminiProvider/provider/default_model
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn test_gemini_provider_creation()`

# Calls

- [new](../../../../../functions/src/llm/provider/gemini/GeminiProvider/new.md)
- [name](../../../../../functions/src/llm/provider/gemini/GeminiProvider/provider/name.md)
- [default_model](../../../../../functions/src/llm/provider/gemini/GeminiProvider/provider/default_model.md)