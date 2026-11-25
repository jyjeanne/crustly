---
type: Rust Function
title: test_invalid_host_falls_back_to_default
resource: src/llm/provider/ollama.rs#L1078-L1082
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/ollama/OllamaProvider/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/OllamaProvider/provider/name
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn test_invalid_host_falls_back_to_default()`

# Calls

- [new](../../../../../functions/src/llm/provider/ollama/OllamaProvider/new.md)
- [name](../../../../../functions/src/llm/provider/ollama/OllamaProvider/provider/name.md)