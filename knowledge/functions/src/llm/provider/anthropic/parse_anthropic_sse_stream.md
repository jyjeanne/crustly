---
type: Rust Function
title: parse_anthropic_sse_stream
resource: src/llm/provider/anthropic.rs#L387-L438
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/anthropic/AnthropicProvider/provider/stream
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/anthropic/sse_stream_yields_every_event_in_a_single_chunk
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/anthropic/sse_stream_reassembles_an_event_split_across_chunks
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_anthropic_sse_stream( byte_stream: impl futures::Stream<Item = reqwest::Result<bytes::Bytes>> + Send + 'static, ) -> impl futures::Stream<Item = Result<StreamEvent>> + Send + 'static`

# Called by

- [stream](../../../../../functions/src/llm/provider/anthropic/AnthropicProvider/provider/stream.md)
- [sse_stream_yields_every_event_in_a_single_chunk](../../../../../functions/src/llm/provider/anthropic/sse_stream_yields_every_event_in_a_single_chunk.md)
- [sse_stream_reassembles_an_event_split_across_chunks](../../../../../functions/src/llm/provider/anthropic/sse_stream_reassembles_an_event_split_across_chunks.md)