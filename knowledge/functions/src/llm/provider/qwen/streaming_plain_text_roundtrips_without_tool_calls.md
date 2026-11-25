---
type: Rust Function
title: streaming_plain_text_roundtrips_without_tool_calls
resource: src/llm/provider/qwen.rs#L1966-L1988
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/qwen/QwenProvider/local
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/qwen/stream_events_from_buffered_content
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn streaming_plain_text_roundtrips_without_tool_calls()`

# Calls

- [local](../../../../../functions/src/llm/provider/qwen/QwenProvider/local.md)
- [stream_events_from_buffered_content](../../../../../functions/src/llm/provider/qwen/stream_events_from_buffered_content.md)