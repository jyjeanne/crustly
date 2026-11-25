---
type: Rust Function
title: llm_response_to_stream_events
resource: src/llm/provider/qwen.rs#L1650-L1723
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/qwen/QwenProvider/provider/stream
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/stream_events_from_buffered_content
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn llm_response_to_stream_events(response: LLMResponse) -> Vec<StreamEvent>`

# Calls

- [push](../../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)

# Called by

- [stream](../../../../../functions/src/llm/provider/qwen/QwenProvider/provider/stream.md)
- [stream_events_from_buffered_content](../../../../../functions/src/llm/provider/qwen/stream_events_from_buffered_content.md)