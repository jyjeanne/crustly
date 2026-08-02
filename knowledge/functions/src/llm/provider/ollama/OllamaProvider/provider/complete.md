---
type: Rust Method
title: complete
resource: src/llm/provider/ollama.rs#L514-L542
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/ollama/OllamaProvider/to_ollama_request
  - functions/src/llm/provider/ollama/OllamaProvider/from_ollama_response
---

# Signature

`async fn complete(&self, request: LLMRequest) -> Result<LLMResponse>`

# Calls

- [to_ollama_request](../../../../../../../functions/src/llm/provider/ollama/OllamaProvider/to_ollama_request.md)
- [from_ollama_response](../../../../../../../functions/src/llm/provider/ollama/OllamaProvider/from_ollama_response.md)