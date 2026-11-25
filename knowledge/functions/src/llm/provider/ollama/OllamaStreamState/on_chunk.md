---
type: Rust Method
title: on_chunk
resource: src/llm/provider/ollama.rs#L755-L797
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/tool_call_recovery/maybe_tool_call_json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaStreamState/open_text_block
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/collect_tool_calls
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/perf_metrics_from_final_data
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/ollama/OllamaProvider/provider/stream
    resolved_by: tree-sitter
    confidence: exact
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

`fn on_chunk(&mut self, chunk: ChatMessageResponse) -> Vec<StreamEvent>`

# Calls

- [maybe_tool_call_json](../../../../../../functions/src/llm/provider/tool_call_recovery/maybe_tool_call_json.md)
- [open_text_block](../../../../../../functions/src/llm/provider/ollama/OllamaStreamState/open_text_block.md)
- [collect_tool_calls](../../../../../../functions/src/llm/provider/ollama/collect_tool_calls.md)
- [perf_metrics_from_final_data](../../../../../../functions/src/llm/provider/ollama/perf_metrics_from_final_data.md)

# Called by

- [stream](../../../../../../functions/src/llm/provider/ollama/OllamaProvider/provider/stream.md)
- [text_is_forwarded_as_each_chunk_arrives](../../../../../../functions/src/llm/provider/ollama/text_is_forwarded_as_each_chunk_arrives.md)
- [usage_comes_from_the_done_chunk](../../../../../../functions/src/llm/provider/ollama/usage_comes_from_the_done_chunk.md)
- [json_after_text_is_streamed_as_text_and_never_run](../../../../../../functions/src/llm/provider/ollama/json_after_text_is_streamed_as_text_and_never_run.md)
- [a_code_block_after_text_streams_immediately](../../../../../../functions/src/llm/provider/ollama/a_code_block_after_text_streams_immediately.md)
- [printed_tool_call_is_withheld_then_recovered](../../../../../../functions/src/llm/provider/ollama/printed_tool_call_is_withheld_then_recovered.md)
- [withheld_text_that_is_not_a_tool_call_is_flushed](../../../../../../functions/src/llm/provider/ollama/withheld_text_that_is_not_a_tool_call_is_flushed.md)