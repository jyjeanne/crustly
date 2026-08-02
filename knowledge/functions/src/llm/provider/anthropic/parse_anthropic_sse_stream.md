---
type: Rust Function
title: parse_anthropic_sse_stream
resource: src/llm/provider/anthropic.rs#L376-L427
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/llm/provider/anthropic/AnthropicProvider/provider/stream
  - functions/src/llm/provider/anthropic/sse_stream_yields_every_event_in_a_single_chunk
  - functions/src/llm/provider/anthropic/sse_stream_reassembles_an_event_split_across_chunks
---

# Signature

`fn parse_anthropic_sse_stream( byte_stream: impl futures::Stream<Item = reqwest::Result<bytes::Bytes>> + Send + 'static, ) -> impl futures::Stream<Item = Result<StreamEvent>> + Send + 'static`

# Calls

- [is_empty](../../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [stream](../../../../../functions/src/llm/provider/anthropic/AnthropicProvider/provider/stream.md)
- [sse_stream_yields_every_event_in_a_single_chunk](../../../../../functions/src/llm/provider/anthropic/sse_stream_yields_every_event_in_a_single_chunk.md)
- [sse_stream_reassembles_an_event_split_across_chunks](../../../../../functions/src/llm/provider/anthropic/sse_stream_reassembles_an_event_split_across_chunks.md)