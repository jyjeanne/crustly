---
type: Rust Function
title: streamed_tool_calls_arrive_before_the_done_chunk
resource: src/llm/provider/ollama.rs#L1367-L1384
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/ollama/collect_tool_calls
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/stop_reason_for
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn streamed_tool_calls_arrive_before_the_done_chunk()`

# Calls

- [collect_tool_calls](../../../../../functions/src/llm/provider/ollama/collect_tool_calls.md)
- [stop_reason_for](../../../../../functions/src/llm/provider/ollama/stop_reason_for.md)