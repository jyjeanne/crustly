---
type: Rust Function
title: invalid_think_value_is_ignored
resource: src/llm/provider/ollama.rs#L1644-L1648
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
  - target: functions/src/llm/provider/types/Message/user
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaProvider/to_ollama_request
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn invalid_think_value_is_ignored()`

# Calls

- [default_local](../../../../../functions/src/llm/provider/ollama/OllamaProvider/default_local.md)
- [with_think](../../../../../functions/src/llm/provider/ollama/OllamaProvider/with_think.md)
- [new](../../../../../functions/src/llm/provider/types/LLMRequest/new.md)
- [user](../../../../../functions/src/llm/provider/types/Message/user.md)
- [to_ollama_request](../../../../../functions/src/llm/provider/ollama/OllamaProvider/to_ollama_request.md)