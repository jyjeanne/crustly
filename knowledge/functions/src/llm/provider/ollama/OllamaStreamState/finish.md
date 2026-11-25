---
type: Rust Method
title: finish
resource: src/llm/provider/ollama.rs#L800-L857
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/tool_call_recovery/tool_call_from_content
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaStreamState/open_text_block
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/stop_reason_for
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/ollama/printed_tool_call_is_withheld_then_recovered
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/withheld_text_that_is_not_a_tool_call_is_flushed
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/usage_comes_from_the_done_chunk
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/json_after_text_is_streamed_as_text_and_never_run
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn finish(mut self) -> Vec<StreamEvent>`

# Calls

- [tool_call_from_content](../../../../../../functions/src/llm/provider/tool_call_recovery/tool_call_from_content.md)
- [open_text_block](../../../../../../functions/src/llm/provider/ollama/OllamaStreamState/open_text_block.md)
- [stop_reason_for](../../../../../../functions/src/llm/provider/ollama/stop_reason_for.md)

# Called by

- [printed_tool_call_is_withheld_then_recovered](../../../../../../functions/src/llm/provider/ollama/printed_tool_call_is_withheld_then_recovered.md)
- [withheld_text_that_is_not_a_tool_call_is_flushed](../../../../../../functions/src/llm/provider/ollama/withheld_text_that_is_not_a_tool_call_is_flushed.md)
- [usage_comes_from_the_done_chunk](../../../../../../functions/src/llm/provider/ollama/usage_comes_from_the_done_chunk.md)
- [json_after_text_is_streamed_as_text_and_never_run](../../../../../../functions/src/llm/provider/ollama/json_after_text_is_streamed_as_text_and_never_run.md)