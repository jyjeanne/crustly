---
type: Rust Function
title: test_fallback_rejects_unregistered_tool_name
resource: src/llm/provider/qwen.rs#L2663-L2672
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/qwen/QwenProvider/local
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/QwenProvider/parse_fallback_tool_calls
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_fallback_rejects_unregistered_tool_name()`

# Calls

- [local](../../../../../functions/src/llm/provider/qwen/QwenProvider/local.md)
- [parse_fallback_tool_calls](../../../../../functions/src/llm/provider/qwen/QwenProvider/parse_fallback_tool_calls.md)