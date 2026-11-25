---
type: Rust Function
title: recovered_tool_call_becomes_a_tool_use_block
resource: src/llm/provider/ollama.rs#L1246-L1270
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
  - target: functions/src/llm/provider/ollama/mock_response
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaProvider/from_ollama_response
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn recovered_tool_call_becomes_a_tool_use_block()`

# Calls

- [default_local](../../../../../functions/src/llm/provider/ollama/OllamaProvider/default_local.md)
- [mock_response](../../../../../functions/src/llm/provider/ollama/mock_response.md)
- [from_ollama_response](../../../../../functions/src/llm/provider/ollama/OllamaProvider/from_ollama_response.md)