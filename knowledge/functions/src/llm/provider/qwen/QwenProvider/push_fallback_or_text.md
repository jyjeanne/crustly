---
type: Rust Method
title: push_fallback_or_text
resource: src/llm/provider/qwen.rs#L954-L981
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/qwen/QwenProvider/parse_fallback_tool_calls
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/qwen/QwenProvider/from_qwen_response
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn push_fallback_or_text( &self, remaining: String, known_tools: &[String], has_tool_calls: &mut bool, content_blocks: &mut Vec<ContentBlock>, )`

# Calls

- [parse_fallback_tool_calls](../../../../../../functions/src/llm/provider/qwen/QwenProvider/parse_fallback_tool_calls.md)

# Called by

- [from_qwen_response](../../../../../../functions/src/llm/provider/qwen/QwenProvider/from_qwen_response.md)