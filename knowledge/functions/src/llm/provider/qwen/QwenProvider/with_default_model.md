---
type: Rust Method
title: with_default_model
resource: src/llm/provider/qwen.rs#L163-L166
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/factory/configure_qwen
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/test_custom_default_model
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn with_default_model(mut self, model: String) -> Self`

# Called by

- [configure_qwen](../../../../../../functions/src/llm/provider/factory/configure_qwen.md)
- [test_custom_default_model](../../../../../../functions/src/llm/provider/qwen/test_custom_default_model.md)