---
type: Rust Function
title: from_ollama_response_falls_back_to_think_tags
resource: src/llm/provider/ollama.rs#L1615-L1632
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/provider/ollama/OllamaProvider/default_local
  - functions/src/llm/provider/types/Message/assistant
  - functions/src/llm/provider/ollama/mock_response
  - functions/src/llm/provider/ollama/OllamaProvider/from_ollama_response
---

# Signature

`fn from_ollama_response_falls_back_to_think_tags()`

# Calls

- [default_local](../../../../../functions/src/llm/provider/ollama/OllamaProvider/default_local.md)
- [assistant](../../../../../functions/src/llm/provider/types/Message/assistant.md)
- [mock_response](../../../../../functions/src/llm/provider/ollama/mock_response.md)
- [from_ollama_response](../../../../../functions/src/llm/provider/ollama/OllamaProvider/from_ollama_response.md)