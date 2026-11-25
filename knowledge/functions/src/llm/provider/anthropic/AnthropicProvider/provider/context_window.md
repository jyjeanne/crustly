---
type: Rust Method
title: context_window
resource: src/llm/provider/anthropic.rs#L318-L331
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/anthropic/test_context_window
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn context_window(&self, model: &str) -> Option<u32>`

# Called by

- [test_context_window](../../../../../../../functions/src/llm/provider/anthropic/test_context_window.md)