---
type: Rust Function
title: from_ollama_response_without_final_data_has_zero_usage_and_no_perf
resource: src/llm/provider/ollama.rs#L1269-L1279
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

`fn from_ollama_response_without_final_data_has_zero_usage_and_no_perf()`

# Calls

- [default_local](../../../../../functions/src/llm/provider/ollama/OllamaProvider/default_local.md)
- [mock_response](../../../../../functions/src/llm/provider/ollama/mock_response.md)
- [assistant](../../../../../functions/src/llm/provider/types/Message/assistant.md)
- [from_ollama_response](../../../../../functions/src/llm/provider/ollama/OllamaProvider/from_ollama_response.md)