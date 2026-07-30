---
type: Rust Function
title: test_streaming_error_handling
resource: tests/streaming_test.rs#L269-L286
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/tests/streaming_test/StreamingMockProvider/with_error
  - functions/src/llm/provider/types/LLMRequest/with_streaming
  - functions/src/tui/events/EventHandler/next
---

# Signature

`async fn test_streaming_error_handling() -> Result<()>`

# Calls

- [with_error](../../../functions/tests/streaming_test/StreamingMockProvider/with_error.md)
- [with_streaming](../../../functions/src/llm/provider/types/LLMRequest/with_streaming.md)
- [next](../../../functions/src/tui/events/EventHandler/next.md)