---
type: Rust Function
title: llm_response_to_stream_events
resource: src/llm/provider/qwen.rs#L1624-L1697
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/llm/provider/qwen/QwenProvider/provider/stream
  - functions/src/llm/provider/qwen/stream_events_from_buffered_content
---

# Signature

`fn llm_response_to_stream_events(response: LLMResponse) -> Vec<StreamEvent>`

# Calls

- [is_empty](../../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [stream](../../../../../functions/src/llm/provider/qwen/QwenProvider/provider/stream.md)
- [stream_events_from_buffered_content](../../../../../functions/src/llm/provider/qwen/stream_events_from_buffered_content.md)