---
type: Rust Function
title: sse
resource: src/llm/provider/openai.rs#L1451-L1453
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/openai/text_is_forwarded_once_its_line_is_complete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/openai/fragmented_tool_call_is_assembled_when_done
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/openai/text_arrives_before_the_response_is_complete
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn sse(value: serde_json::Value) -> String`

# Called by

- [text_is_forwarded_once_its_line_is_complete](../../../../../functions/src/llm/provider/openai/text_is_forwarded_once_its_line_is_complete.md)
- [fragmented_tool_call_is_assembled_when_done](../../../../../functions/src/llm/provider/openai/fragmented_tool_call_is_assembled_when_done.md)
- [text_arrives_before_the_response_is_complete](../../../../../functions/src/llm/provider/openai/text_arrives_before_the_response_is_complete.md)