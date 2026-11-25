---
type: Rust Method
title: context_window
resource: src/llm/provider/ollama.rs#L656-L672
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/ollama/OllamaProvider/overrides_for
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/ollama/context_window_reflects_the_per_model_num_ctx_that_is_actually_requested
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/test_context_window_default_and_custom
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn context_window(&self, model: &str) -> Option<u32>`

# Calls

- [overrides_for](../../../../../../../functions/src/llm/provider/ollama/OllamaProvider/overrides_for.md)

# Called by

- [context_window_reflects_the_per_model_num_ctx_that_is_actually_requested](../../../../../../../functions/src/llm/provider/ollama/context_window_reflects_the_per_model_num_ctx_that_is_actually_requested.md)
- [test_context_window_default_and_custom](../../../../../../../functions/src/llm/provider/ollama/test_context_window_default_and_custom.md)