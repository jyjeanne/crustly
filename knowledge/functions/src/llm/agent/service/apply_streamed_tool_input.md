---
type: Rust Function
title: apply_streamed_tool_input
resource: src/llm/agent/service.rs#L380-L405
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/drain_stream_to_response
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn apply_streamed_tool_input(block: ContentBlock, json_buf: &str) -> ContentBlock`

# Called by

- [drain_stream_to_response](../../../../../functions/src/llm/agent/service/drain_stream_to_response.md)