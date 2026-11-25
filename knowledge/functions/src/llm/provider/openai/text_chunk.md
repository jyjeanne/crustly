---
type: Rust Function
title: text_chunk
resource: src/llm/provider/openai.rs#L1455-L1461
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/openai/text_is_forwarded_once_its_line_is_complete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/openai/text_arrives_before_the_response_is_complete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/openai/last_line_without_newline_is_parsed_at_the_end
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn text_chunk(text: &str, finish_reason: Option<&str>) -> serde_json::Value`

# Called by

- [text_is_forwarded_once_its_line_is_complete](../../../../../functions/src/llm/provider/openai/text_is_forwarded_once_its_line_is_complete.md)
- [text_arrives_before_the_response_is_complete](../../../../../functions/src/llm/provider/openai/text_arrives_before_the_response_is_complete.md)
- [last_line_without_newline_is_parsed_at_the_end](../../../../../functions/src/llm/provider/openai/last_line_without_newline_is_parsed_at_the_end.md)