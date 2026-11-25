---
type: Rust Function
title: test_context_window_all_known_models
resource: src/llm/provider/gemini.rs#L1182-L1200
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
  - target: functions/src/llm/provider/gemini/GeminiProvider/provider/context_window
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn test_context_window_all_known_models()`

# Calls

- [new](../../../../../functions/src/llm/provider/gemini/GeminiProvider/new.md)
- [context_window](../../../../../functions/src/llm/provider/gemini/GeminiProvider/provider/context_window.md)