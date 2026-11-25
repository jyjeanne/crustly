---
type: Rust Method
title: on_bytes
resource: src/llm/provider/openai.rs#L994-L1013
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/openai/OpenAIStreamState/on_line
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/openai/OpenAIProvider/provider/stream
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/openai/text_is_forwarded_once_its_line_is_complete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/openai/fragmented_tool_call_is_assembled_when_done
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/openai/last_line_without_newline_is_parsed_at_the_end
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn on_bytes(&mut self, bytes: &[u8]) -> Vec<StreamEvent>`

# Calls

- [on_line](../../../../../../functions/src/llm/provider/openai/OpenAIStreamState/on_line.md)

# Called by

- [stream](../../../../../../functions/src/llm/provider/openai/OpenAIProvider/provider/stream.md)
- [text_is_forwarded_once_its_line_is_complete](../../../../../../functions/src/llm/provider/openai/text_is_forwarded_once_its_line_is_complete.md)
- [fragmented_tool_call_is_assembled_when_done](../../../../../../functions/src/llm/provider/openai/fragmented_tool_call_is_assembled_when_done.md)
- [last_line_without_newline_is_parsed_at_the_end](../../../../../../functions/src/llm/provider/openai/last_line_without_newline_is_parsed_at_the_end.md)