---
type: Rust Method
title: complete
resource: src/llm/provider/ollama.rs#L519-L547
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/ollama/OllamaProvider/to_ollama_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaProvider/from_ollama_response
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn complete(&self, request: LLMRequest) -> Result<LLMResponse>`

# Calls

- [to_ollama_request](../../../../../../../functions/src/llm/provider/ollama/OllamaProvider/to_ollama_request.md)
- [from_ollama_response](../../../../../../../functions/src/llm/provider/ollama/OllamaProvider/from_ollama_response.md)