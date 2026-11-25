---
type: Rust Function
title: request_thinking_wins_over_configured_think
resource: src/llm/provider/ollama.rs#L1635-L1640
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/ollama/OllamaProvider/default_local
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaProvider/with_think
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/LLMRequest/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/types/LLMRequest/with_thinking
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/OllamaProvider/to_ollama_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/Message/user
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn request_thinking_wins_over_configured_think()`

# Calls

- [default_local](../../../../../functions/src/llm/provider/ollama/OllamaProvider/default_local.md)
- [with_think](../../../../../functions/src/llm/provider/ollama/OllamaProvider/with_think.md)
- [new](../../../../../functions/src/llm/provider/types/LLMRequest/new.md)
- [with_thinking](../../../../../functions/src/llm/provider/types/LLMRequest/with_thinking.md)
- [to_ollama_request](../../../../../functions/src/llm/provider/ollama/OllamaProvider/to_ollama_request.md)
- [user](../../../../../functions/src/llm/provider/types/Message/user.md)