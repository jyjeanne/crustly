---
type: Rust Function
title: from_ollama_response_without_final_data_has_zero_usage_and_no_perf
resource: src/llm/provider/ollama.rs#L1327-L1337
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

`fn from_ollama_response_without_final_data_has_zero_usage_and_no_perf()`

# Calls

- [default_local](../../../../../functions/src/llm/provider/ollama/OllamaProvider/default_local.md)
- [mock_response](../../../../../functions/src/llm/provider/ollama/mock_response.md)
- [from_ollama_response](../../../../../functions/src/llm/provider/ollama/OllamaProvider/from_ollama_response.md)