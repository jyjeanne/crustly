---
type: Rust Function
title: test_streaming_stop_reason
resource: tests/streaming_test.rs#L251-L266
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/provider/types/LLMRequest/with_streaming
  - functions/src/tui/events/EventHandler/next
---

# Signature

`async fn test_streaming_stop_reason() -> Result<()>`

# Calls

- [with_streaming](../../../functions/src/llm/provider/types/LLMRequest/with_streaming.md)
- [next](../../../functions/src/tui/events/EventHandler/next.md)