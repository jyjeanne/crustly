---
type: Rust Function
title: sse_stream_yields_every_event_in_a_single_chunk
resource: src/llm/provider/anthropic.rs#L575-L597
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/anthropic/parse_anthropic_sse_stream
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn sse_stream_yields_every_event_in_a_single_chunk()`

# Calls

- [parse_anthropic_sse_stream](../../../../../functions/src/llm/provider/anthropic/parse_anthropic_sse_stream.md)