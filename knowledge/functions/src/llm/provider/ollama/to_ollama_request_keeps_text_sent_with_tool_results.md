---
type: Rust Function
title: to_ollama_request_keeps_text_sent_with_tool_results
resource: src/llm/provider/ollama.rs#L1568-L1592
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
  - target: functions/src/llm/provider/ollama/OllamaProvider/to_ollama_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn to_ollama_request_keeps_text_sent_with_tool_results()`

# Calls

- [default_local](../../../../../functions/src/llm/provider/ollama/OllamaProvider/default_local.md)
- [new](../../../../../functions/src/llm/provider/types/LLMRequest/new.md)
- [to_ollama_request](../../../../../functions/src/llm/provider/ollama/OllamaProvider/to_ollama_request.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)