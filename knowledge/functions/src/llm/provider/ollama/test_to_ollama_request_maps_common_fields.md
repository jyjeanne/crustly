---
type: Rust Function
title: test_to_ollama_request_maps_common_fields
resource: src/llm/provider/ollama.rs#L1136-L1152
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/ollama/OllamaProvider/default_local
  - functions/src/llm/provider/types/LLMRequest/with_system
  - functions/src/llm/provider/types/LLMRequest/with_temperature
  - functions/src/llm/provider/types/LLMRequest/with_top_p
  - functions/src/llm/provider/types/LLMRequest/with_seed
  - functions/src/llm/provider/types/LLMRequest/with_stop
  - functions/src/llm/provider/types/LLMRequest/with_max_tokens
  - functions/src/llm/provider/ollama/OllamaProvider/to_ollama_request
---

# Signature

`fn test_to_ollama_request_maps_common_fields()`

# Calls

- [default_local](../../../../../functions/src/llm/provider/ollama/OllamaProvider/default_local.md)
- [with_system](../../../../../functions/src/llm/provider/types/LLMRequest/with_system.md)
- [with_temperature](../../../../../functions/src/llm/provider/types/LLMRequest/with_temperature.md)
- [with_top_p](../../../../../functions/src/llm/provider/types/LLMRequest/with_top_p.md)
- [with_seed](../../../../../functions/src/llm/provider/types/LLMRequest/with_seed.md)
- [with_stop](../../../../../functions/src/llm/provider/types/LLMRequest/with_stop.md)
- [with_max_tokens](../../../../../functions/src/llm/provider/types/LLMRequest/with_max_tokens.md)
- [to_ollama_request](../../../../../functions/src/llm/provider/ollama/OllamaProvider/to_ollama_request.md)