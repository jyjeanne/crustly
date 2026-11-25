---
type: Rust Function
title: context_window_reflects_the_per_model_num_ctx_that_is_actually_requested
resource: src/llm/provider/ollama.rs#L1051-L1075
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/ollama/ModelOverrides/from_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaProvider/default_local
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaProvider/with_num_ctx
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaProvider/with_per_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaProvider/provider/context_window
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn context_window_reflects_the_per_model_num_ctx_that_is_actually_requested()`

# Calls

- [from_config](../../../../../functions/src/llm/provider/ollama/ModelOverrides/from_config.md)
- [default_local](../../../../../functions/src/llm/provider/ollama/OllamaProvider/default_local.md)
- [with_num_ctx](../../../../../functions/src/llm/provider/ollama/OllamaProvider/with_num_ctx.md)
- [with_per_model](../../../../../functions/src/llm/provider/ollama/OllamaProvider/with_per_model.md)
- [context_window](../../../../../functions/src/llm/provider/ollama/OllamaProvider/provider/context_window.md)