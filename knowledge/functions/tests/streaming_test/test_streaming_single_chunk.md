---
type: Rust Function
title: test_streaming_single_chunk
resource: tests/streaming_test.rs#L161-L183
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/types/LLMRequest/with_streaming
  - functions/src/tui/events/EventHandler/next
---

# Signature

`async fn test_streaming_single_chunk() -> Result<()>`

# Calls

- [with_streaming](../../../functions/src/llm/provider/types/LLMRequest/with_streaming.md)
- [next](../../../functions/src/tui/events/EventHandler/next.md)