---
type: Rust Function
title: stream_events_from_buffered_content
resource: src/llm/provider/qwen.rs#L1908-L1933
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/qwen/QwenProvider/from_qwen_response
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/llm_response_to_stream_events
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/qwen/streaming_assembles_hermes_tool_call_from_buffered_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/streaming_plain_text_roundtrips_without_tool_calls
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn stream_events_from_buffered_content( provider: &QwenProvider, content: &str, known_tools: &[String], ) -> Vec<StreamEvent>`

# Calls

- [from_qwen_response](../../../../../functions/src/llm/provider/qwen/QwenProvider/from_qwen_response.md)
- [llm_response_to_stream_events](../../../../../functions/src/llm/provider/qwen/llm_response_to_stream_events.md)

# Called by

- [streaming_assembles_hermes_tool_call_from_buffered_text](../../../../../functions/src/llm/provider/qwen/streaming_assembles_hermes_tool_call_from_buffered_text.md)
- [streaming_plain_text_roundtrips_without_tool_calls](../../../../../functions/src/llm/provider/qwen/streaming_plain_text_roundtrips_without_tool_calls.md)