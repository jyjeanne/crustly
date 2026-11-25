---
type: Rust Function
title: chunk
resource: src/llm/provider/ollama.rs#L1688-L1697
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/ollama/text_is_forwarded_as_each_chunk_arrives
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/usage_comes_from_the_done_chunk
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/json_after_text_is_streamed_as_text_and_never_run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/a_code_block_after_text_streams_immediately
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/printed_tool_call_is_withheld_then_recovered
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/withheld_text_that_is_not_a_tool_call_is_flushed
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn chunk(content: &str, done: bool) -> ChatMessageResponse`

# Called by

- [text_is_forwarded_as_each_chunk_arrives](../../../../../functions/src/llm/provider/ollama/text_is_forwarded_as_each_chunk_arrives.md)
- [usage_comes_from_the_done_chunk](../../../../../functions/src/llm/provider/ollama/usage_comes_from_the_done_chunk.md)
- [json_after_text_is_streamed_as_text_and_never_run](../../../../../functions/src/llm/provider/ollama/json_after_text_is_streamed_as_text_and_never_run.md)
- [a_code_block_after_text_streams_immediately](../../../../../functions/src/llm/provider/ollama/a_code_block_after_text_streams_immediately.md)
- [printed_tool_call_is_withheld_then_recovered](../../../../../functions/src/llm/provider/ollama/printed_tool_call_is_withheld_then_recovered.md)
- [withheld_text_that_is_not_a_tool_call_is_flushed](../../../../../functions/src/llm/provider/ollama/withheld_text_that_is_not_a_tool_call_is_flushed.md)