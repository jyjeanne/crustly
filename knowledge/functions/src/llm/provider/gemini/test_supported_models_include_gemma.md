---
type: Rust Function
title: test_supported_models_include_gemma
resource: src/llm/provider/gemini.rs#L916-L922
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
  - target: functions/src/llm/provider/gemini/GeminiProvider/provider/supported_models
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn test_supported_models_include_gemma()`

# Calls

- [new](../../../../../functions/src/llm/provider/gemini/GeminiProvider/new.md)
- [supported_models](../../../../../functions/src/llm/provider/gemini/GeminiProvider/provider/supported_models.md)