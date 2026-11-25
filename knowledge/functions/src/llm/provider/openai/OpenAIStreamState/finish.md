---
type: Rust Method
title: finish
resource: src/llm/provider/openai.rs#L1120-L1213
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/openai/OpenAIStreamState/on_line
    resolved_by: tree-sitter
    confidence: exact
  called_by:
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

`fn finish(mut self) -> Vec<StreamEvent>`

# Calls

- [on_line](../../../../../../functions/src/llm/provider/openai/OpenAIStreamState/on_line.md)

# Called by

- [fragmented_tool_call_is_assembled_when_done](../../../../../../functions/src/llm/provider/openai/fragmented_tool_call_is_assembled_when_done.md)
- [last_line_without_newline_is_parsed_at_the_end](../../../../../../functions/src/llm/provider/openai/last_line_without_newline_is_parsed_at_the_end.md)
- [empty_stream_still_starts_and_stops_the_message](../../../../../../functions/src/llm/provider/openai/empty_stream_still_starts_and_stops_the_message.md)