---
type: Rust Function
title: sse_stream_reassembles_an_event_split_across_chunks
resource: src/llm/provider/anthropic.rs#L604-L624
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/anthropic/parse_anthropic_sse_stream
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn sse_stream_reassembles_an_event_split_across_chunks()`

# Calls

- [find](../../../../../functions/src/memory/MemoryStore/find.md)
- [parse_anthropic_sse_stream](../../../../../functions/src/llm/provider/anthropic/parse_anthropic_sse_stream.md)