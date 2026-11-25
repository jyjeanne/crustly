---
type: Rust Method
title: context_window
resource: src/llm/provider/gemini.rs#L689-L702
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/gemini/test_context_window
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/gemini/test_context_window_all_known_models
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn context_window(&self, model: &str) -> Option<u32>`

# Called by

- [test_context_window](../../../../../../../functions/src/llm/provider/gemini/test_context_window.md)
- [test_context_window_all_known_models](../../../../../../../functions/src/llm/provider/gemini/test_context_window_all_known_models.md)