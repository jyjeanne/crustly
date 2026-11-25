---
type: Rust Function
title: test_clean_incomplete_markers
resource: src/llm/provider/qwen.rs#L2626-L2639
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
  - target: functions/src/llm/provider/qwen/QwenProvider/with_tool_parser
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/clean_incomplete_markers
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_clean_incomplete_markers()`

# Calls

- [local](../../../../../functions/src/llm/provider/qwen/QwenProvider/local.md)
- [with_tool_parser](../../../../../functions/src/llm/provider/qwen/QwenProvider/with_tool_parser.md)
- [clean_incomplete_markers](../../../../../functions/src/llm/provider/qwen/QwenProvider/clean_incomplete_markers.md)