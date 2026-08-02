---
type: Rust Function
title: recovered_tool_call_becomes_a_tool_use_block
resource: src/llm/provider/ollama.rs#L1188-L1212
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/ollama/OllamaProvider/default_local
  - functions/src/llm/provider/ollama/mock_response
  - functions/src/llm/provider/types/Message/assistant
  - functions/src/llm/provider/ollama/OllamaProvider/from_ollama_response
---

# Signature

`fn recovered_tool_call_becomes_a_tool_use_block()`

# Calls

- [default_local](../../../../../functions/src/llm/provider/ollama/OllamaProvider/default_local.md)
- [mock_response](../../../../../functions/src/llm/provider/ollama/mock_response.md)
- [assistant](../../../../../functions/src/llm/provider/types/Message/assistant.md)
- [from_ollama_response](../../../../../functions/src/llm/provider/ollama/OllamaProvider/from_ollama_response.md)