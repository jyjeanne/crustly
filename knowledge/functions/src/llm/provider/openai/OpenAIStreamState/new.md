---
type: Rust Method
title: new
resource: src/llm/provider/openai.rs#L980-L991
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/openai/OpenAIProvider/provider/stream
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/openai/text_is_forwarded_once_its_line_is_complete
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/openai/fragmented_tool_call_is_assembled_when_done
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/openai/last_line_without_newline_is_parsed_at_the_end
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/openai/empty_stream_still_starts_and_stops_the_message
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn new(request_model: String) -> Self`

# Called by

- [stream](../../../../../../functions/src/llm/provider/openai/OpenAIProvider/provider/stream.md)
- [text_is_forwarded_once_its_line_is_complete](../../../../../../functions/src/llm/provider/openai/text_is_forwarded_once_its_line_is_complete.md)
- [fragmented_tool_call_is_assembled_when_done](../../../../../../functions/src/llm/provider/openai/fragmented_tool_call_is_assembled_when_done.md)
- [last_line_without_newline_is_parsed_at_the_end](../../../../../../functions/src/llm/provider/openai/last_line_without_newline_is_parsed_at_the_end.md)
- [empty_stream_still_starts_and_stops_the_message](../../../../../../functions/src/llm/provider/openai/empty_stream_still_starts_and_stops_the_message.md)