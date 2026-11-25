---
type: Rust Function
title: to_ollama_request_maps_thinking_and_response_format
resource: src/llm/provider/ollama.rs#L1595-L1604
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
  - target: functions/src/llm/provider/types/LLMRequest/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/types/LLMRequest/with_thinking
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/types/LLMRequest/with_response_format
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaProvider/to_ollama_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/Message/user
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn to_ollama_request_maps_thinking_and_response_format()`

# Calls

- [default_local](../../../../../functions/src/llm/provider/ollama/OllamaProvider/default_local.md)
- [new](../../../../../functions/src/llm/provider/types/LLMRequest/new.md)
- [with_thinking](../../../../../functions/src/llm/provider/types/LLMRequest/with_thinking.md)
- [with_response_format](../../../../../functions/src/llm/provider/types/LLMRequest/with_response_format.md)
- [to_ollama_request](../../../../../functions/src/llm/provider/ollama/OllamaProvider/to_ollama_request.md)
- [user](../../../../../functions/src/llm/provider/types/Message/user.md)