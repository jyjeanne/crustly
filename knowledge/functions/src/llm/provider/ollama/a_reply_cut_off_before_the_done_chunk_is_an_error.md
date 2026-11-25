---
type: Rust Function
title: a_reply_cut_off_before_the_done_chunk_is_an_error
resource: src/llm/provider/ollama.rs#L1930-L1966
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
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn a_reply_cut_off_before_the_done_chunk_is_an_error()`

# Calls

- [held_response_server](../../../../../functions/src/llm/provider/test_support/held_response_server.md)
- [new](../../../../../functions/src/llm/provider/ollama/OllamaProvider/new.md)
- [new](../../../../../functions/src/llm/provider/types/LLMRequest/new.md)
- [stream](../../../../../functions/src/llm/provider/ollama/OllamaProvider/provider/stream.md)
- [next](../../../../../functions/src/tui/events/EventHandler/next.md)
- [user](../../../../../functions/src/llm/provider/types/Message/user.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)