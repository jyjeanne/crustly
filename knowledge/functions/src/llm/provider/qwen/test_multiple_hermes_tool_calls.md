---
type: Rust Function
title: test_multiple_hermes_tool_calls
resource: src/llm/provider/qwen.rs#L2164-L2179
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
  - target: functions/src/llm/provider/qwen/QwenProvider/parse_hermes_tool_calls
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_multiple_hermes_tool_calls()`

# Calls

- [local](../../../../../functions/src/llm/provider/qwen/QwenProvider/local.md)
- [parse_hermes_tool_calls](../../../../../functions/src/llm/provider/qwen/QwenProvider/parse_hermes_tool_calls.md)