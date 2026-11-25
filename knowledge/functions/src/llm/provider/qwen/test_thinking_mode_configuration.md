---
type: Rust Function
title: test_thinking_mode_configuration
resource: src/llm/provider/qwen.rs#L2140-L2146
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/qwen/QwenProvider/local
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/QwenProvider/with_thinking
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/QwenProvider/with_thinking_budget
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_thinking_mode_configuration()`

# Calls

- [local](../../../../../functions/src/llm/provider/qwen/QwenProvider/local.md)
- [with_thinking](../../../../../functions/src/llm/provider/qwen/QwenProvider/with_thinking.md)
- [with_thinking_budget](../../../../../functions/src/llm/provider/qwen/QwenProvider/with_thinking_budget.md)