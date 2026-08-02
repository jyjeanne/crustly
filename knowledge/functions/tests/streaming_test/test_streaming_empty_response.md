---
type: Rust Function
title: test_streaming_empty_response
resource: tests/streaming_test.rs#L289-L305
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/types/LLMRequest/with_streaming
  - functions/src/tui/events/EventHandler/next
---

# Signature

`async fn test_streaming_empty_response() -> Result<()>`

# Calls

- [with_streaming](../../../functions/src/llm/provider/types/LLMRequest/with_streaming.md)
- [next](../../../functions/src/tui/events/EventHandler/next.md)