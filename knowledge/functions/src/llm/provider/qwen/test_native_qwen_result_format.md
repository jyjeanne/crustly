---
type: Rust Function
title: test_native_qwen_result_format
resource: src/llm/provider/qwen.rs#L2615-L2623
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
  - target: functions/src/llm/provider/qwen/QwenProvider/format_native_qwen_result
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_native_qwen_result_format()`

# Calls

- [local](../../../../../functions/src/llm/provider/qwen/QwenProvider/local.md)
- [with_tool_parser](../../../../../functions/src/llm/provider/qwen/QwenProvider/with_tool_parser.md)
- [format_native_qwen_result](../../../../../functions/src/llm/provider/qwen/QwenProvider/format_native_qwen_result.md)