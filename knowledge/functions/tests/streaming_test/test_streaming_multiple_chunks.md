---
type: Rust Function
title: test_streaming_multiple_chunks
resource: tests/streaming_test.rs#L186-L215
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/types/LLMRequest/with_streaming
  - functions/src/tui/events/EventHandler/next
---

# Signature

`async fn test_streaming_multiple_chunks() -> Result<()>`

# Calls

- [with_streaming](../../../functions/src/llm/provider/types/LLMRequest/with_streaming.md)
- [next](../../../functions/src/tui/events/EventHandler/next.md)