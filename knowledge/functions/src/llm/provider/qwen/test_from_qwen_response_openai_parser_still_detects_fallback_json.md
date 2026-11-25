---
type: Rust Function
title: test_from_qwen_response_openai_parser_still_detects_fallback_json
resource: src/llm/provider/qwen.rs#L2784-L2819
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
  - target: functions/src/llm/provider/qwen/QwenProvider/from_qwen_response
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_from_qwen_response_openai_parser_still_detects_fallback_json()`

# Calls

- [local](../../../../../functions/src/llm/provider/qwen/QwenProvider/local.md)
- [with_tool_parser](../../../../../functions/src/llm/provider/qwen/QwenProvider/with_tool_parser.md)
- [from_qwen_response](../../../../../functions/src/llm/provider/qwen/QwenProvider/from_qwen_response.md)