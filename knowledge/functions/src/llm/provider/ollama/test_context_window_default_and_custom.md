---
type: Rust Function
title: test_context_window_default_and_custom
resource: src/llm/provider/ollama.rs#L1106-L1112
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/ollama/OllamaProvider/default_local
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaProvider/with_num_ctx
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaProvider/provider/context_window
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn test_context_window_default_and_custom()`

# Calls

- [default_local](../../../../../functions/src/llm/provider/ollama/OllamaProvider/default_local.md)
- [with_num_ctx](../../../../../functions/src/llm/provider/ollama/OllamaProvider/with_num_ctx.md)
- [context_window](../../../../../functions/src/llm/provider/ollama/OllamaProvider/provider/context_window.md)