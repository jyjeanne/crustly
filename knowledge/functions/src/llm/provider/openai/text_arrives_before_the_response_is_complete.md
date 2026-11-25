---
type: Rust Function
title: text_arrives_before_the_response_is_complete
resource: src/llm/provider/openai.rs#L1575-L1615
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/test_support/held_response_server
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/openai/sse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/openai/text_chunk
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/openai/OpenAIProvider/local
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/types/LLMRequest/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/openai/OpenAIProvider/provider/stream
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/events/EventHandler/next
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/Message/user
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn text_arrives_before_the_response_is_complete()`

# Calls

- [held_response_server](../../../../../functions/src/llm/provider/test_support/held_response_server.md)
- [sse](../../../../../functions/src/llm/provider/openai/sse.md)
- [text_chunk](../../../../../functions/src/llm/provider/openai/text_chunk.md)
- [local](../../../../../functions/src/llm/provider/openai/OpenAIProvider/local.md)
- [new](../../../../../functions/src/llm/provider/types/LLMRequest/new.md)
- [stream](../../../../../functions/src/llm/provider/openai/OpenAIProvider/provider/stream.md)
- [next](../../../../../functions/src/tui/events/EventHandler/next.md)
- [user](../../../../../functions/src/llm/provider/types/Message/user.md)