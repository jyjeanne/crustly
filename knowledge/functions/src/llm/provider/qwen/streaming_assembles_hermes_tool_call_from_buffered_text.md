---
type: Rust Function
title: streaming_assembles_hermes_tool_call_from_buffered_text
resource: src/llm/provider/qwen.rs#L1938-L1962
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
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

`fn streaming_assembles_hermes_tool_call_from_buffered_text()`

# Calls

- [local](../../../../../functions/src/llm/provider/qwen/QwenProvider/local.md)
- [stream_events_from_buffered_content](../../../../../functions/src/llm/provider/qwen/stream_events_from_buffered_content.md)