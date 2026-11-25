---
type: Rust Method
title: context_window
resource: src/llm/provider/openai.rs#L718-L731
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/openai/test_context_window
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn context_window(&self, model: &str) -> Option<u32>`

# Called by

- [test_context_window](../../../../../../../functions/src/llm/provider/openai/test_context_window.md)