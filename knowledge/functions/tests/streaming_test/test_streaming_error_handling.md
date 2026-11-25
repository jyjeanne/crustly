---
type: Rust Function
title: test_streaming_error_handling
resource: tests/streaming_test.rs#L269-L286
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/tests/streaming_test/StreamingMockProvider/with_error
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/LLMRequest/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/types/LLMRequest/with_streaming
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/streaming_test/StreamingMockProvider/provider/stream
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/events/EventHandler/next
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_streaming_error_handling() -> Result<()>`

# Calls

- [with_error](../../../functions/tests/streaming_test/StreamingMockProvider/with_error.md)
- [new](../../../functions/src/llm/provider/types/LLMRequest/new.md)
- [with_streaming](../../../functions/src/llm/provider/types/LLMRequest/with_streaming.md)
- [stream](../../../functions/tests/streaming_test/StreamingMockProvider/provider/stream.md)
- [next](../../../functions/src/tui/events/EventHandler/next.md)