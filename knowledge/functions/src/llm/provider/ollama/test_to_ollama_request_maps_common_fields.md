---
type: Rust Function
title: test_to_ollama_request_maps_common_fields
resource: src/llm/provider/ollama.rs#L1194-L1210
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/ollama/OllamaProvider/default_local
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/LLMRequest/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/types/LLMRequest/with_system
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/LLMRequest/with_temperature
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/LLMRequest/with_top_p
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/LLMRequest/with_seed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/LLMRequest/with_stop
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/LLMRequest/with_max_tokens
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaProvider/to_ollama_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/Message/user
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_to_ollama_request_maps_common_fields()`

# Calls

- [default_local](../../../../../functions/src/llm/provider/ollama/OllamaProvider/default_local.md)
- [new](../../../../../functions/src/llm/provider/types/LLMRequest/new.md)
- [with_system](../../../../../functions/src/llm/provider/types/LLMRequest/with_system.md)
- [with_temperature](../../../../../functions/src/llm/provider/types/LLMRequest/with_temperature.md)
- [with_top_p](../../../../../functions/src/llm/provider/types/LLMRequest/with_top_p.md)
- [with_seed](../../../../../functions/src/llm/provider/types/LLMRequest/with_seed.md)
- [with_stop](../../../../../functions/src/llm/provider/types/LLMRequest/with_stop.md)
- [with_max_tokens](../../../../../functions/src/llm/provider/types/LLMRequest/with_max_tokens.md)
- [to_ollama_request](../../../../../functions/src/llm/provider/ollama/OllamaProvider/to_ollama_request.md)
- [user](../../../../../functions/src/llm/provider/types/Message/user.md)