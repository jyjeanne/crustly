---
type: Rust Method
title: context_window
resource: src/llm/provider/qwen.rs#L1580-L1610
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/qwen/QwenProvider/is_local
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/qwen/test_context_window
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn context_window(&self, model: &str) -> Option<u32>`

# Calls

- [is_local](../../../../../../../functions/src/llm/provider/qwen/QwenProvider/is_local.md)

# Called by

- [test_context_window](../../../../../../../functions/src/llm/provider/qwen/test_context_window.md)