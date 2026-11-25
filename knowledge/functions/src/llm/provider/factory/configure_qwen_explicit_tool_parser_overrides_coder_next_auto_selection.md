---
type: Rust Function
title: configure_qwen_explicit_tool_parser_overrides_coder_next_auto_selection
resource: src/llm/provider/factory.rs#L1193-L1214
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/factory/configure_qwen
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/local
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/QwenProvider/tool_parser
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn configure_qwen_explicit_tool_parser_overrides_coder_next_auto_selection()`

# Calls

- [configure_qwen](../../../../../functions/src/llm/provider/factory/configure_qwen.md)
- [local](../../../../../functions/src/llm/provider/qwen/QwenProvider/local.md)
- [tool_parser](../../../../../functions/src/llm/provider/qwen/QwenProvider/tool_parser.md)