---
type: Rust Method
title: with_thinking_budget
resource: src/llm/provider/qwen.rs#L208-L211
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/provider/factory/configure_qwen
  - functions/src/llm/provider/qwen/test_thinking_mode_configuration
---

# Signature

`pub fn with_thinking_budget(mut self, budget_tokens: u32) -> Self`

# Called by

- [configure_qwen](../../../../../../functions/src/llm/provider/factory/configure_qwen.md)
- [test_thinking_mode_configuration](../../../../../../functions/src/llm/provider/qwen/test_thinking_mode_configuration.md)