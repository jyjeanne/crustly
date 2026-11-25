---
type: Rust Function
title: test_native_qwen_tool_call_parsing
resource: src/llm/provider/qwen.rs#L2558-L2571
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
  - target: functions/src/llm/provider/qwen/QwenProvider/with_tool_parser
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/parse_native_qwen_tool_calls
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_native_qwen_tool_call_parsing()`

# Calls

- [local](../../../../../functions/src/llm/provider/qwen/QwenProvider/local.md)
- [with_tool_parser](../../../../../functions/src/llm/provider/qwen/QwenProvider/with_tool_parser.md)
- [parse_native_qwen_tool_calls](../../../../../functions/src/llm/provider/qwen/QwenProvider/parse_native_qwen_tool_calls.md)