---
type: Rust Function
title: test_hermes_malformed_json_is_skipped_without_panicking
resource: src/llm/provider/qwen.rs#L2184-L2194
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
  - target: functions/src/llm/provider/qwen/QwenProvider/parse_hermes_tool_calls
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_hermes_malformed_json_is_skipped_without_panicking()`

# Calls

- [local](../../../../../functions/src/llm/provider/qwen/QwenProvider/local.md)
- [parse_hermes_tool_calls](../../../../../functions/src/llm/provider/qwen/QwenProvider/parse_hermes_tool_calls.md)