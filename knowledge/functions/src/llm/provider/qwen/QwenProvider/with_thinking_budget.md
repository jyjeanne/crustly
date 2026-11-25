---
type: Rust Method
title: with_thinking_budget
resource: src/llm/provider/qwen.rs#L208-L211
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/factory/configure_qwen
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_thinking_mode_configuration
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn with_thinking_budget(mut self, budget_tokens: u32) -> Self`

# Called by

- [configure_qwen](../../../../../../functions/src/llm/provider/factory/configure_qwen.md)
- [test_thinking_mode_configuration](../../../../../../functions/src/llm/provider/qwen/test_thinking_mode_configuration.md)