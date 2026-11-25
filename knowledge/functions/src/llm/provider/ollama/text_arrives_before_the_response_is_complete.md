---
type: Rust Function
title: text_arrives_before_the_response_is_complete
resource: src/llm/provider/ollama.rs#L1820-L1882
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
  - target: functions/src/memory/recall/line
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaProvider/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/types/LLMRequest/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/OllamaProvider/provider/stream
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/events/EventHandler/next
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/Message/user
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/stop_reason
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn text_arrives_before_the_response_is_complete()`

# Calls

- [held_response_server](../../../../../functions/src/llm/provider/test_support/held_response_server.md)
- [line](../../../../../functions/src/memory/recall/line.md)
- [new](../../../../../functions/src/llm/provider/ollama/OllamaProvider/new.md)
- [new](../../../../../functions/src/llm/provider/types/LLMRequest/new.md)
- [stream](../../../../../functions/src/llm/provider/ollama/OllamaProvider/provider/stream.md)
- [next](../../../../../functions/src/tui/events/EventHandler/next.md)
- [user](../../../../../functions/src/llm/provider/types/Message/user.md)
- [stop_reason](../../../../../functions/src/llm/provider/ollama/stop_reason.md)