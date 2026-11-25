---
type: Rust Function
title: test_streaming_basic
resource: tests/streaming_test.rs#L124-L158
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/tests/streaming_test/StreamingMockProvider/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
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
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_streaming_basic() -> Result<()>`

# Calls

- [new](../../../functions/tests/streaming_test/StreamingMockProvider/new.md)
- [new](../../../functions/src/llm/provider/types/LLMRequest/new.md)
- [with_streaming](../../../functions/src/llm/provider/types/LLMRequest/with_streaming.md)
- [stream](../../../functions/tests/streaming_test/StreamingMockProvider/provider/stream.md)
- [next](../../../functions/src/tui/events/EventHandler/next.md)
- [push](../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)