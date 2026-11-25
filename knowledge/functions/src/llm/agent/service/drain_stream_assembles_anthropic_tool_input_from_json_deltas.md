---
type: Rust Function
title: drain_stream_assembles_anthropic_tool_input_from_json_deltas
resource: src/llm/agent/service.rs#L2412-L2475
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/drain_stream_to_response
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn drain_stream_assembles_anthropic_tool_input_from_json_deltas()`

# Calls

- [drain_stream_to_response](../../../../../functions/src/llm/agent/service/drain_stream_to_response.md)