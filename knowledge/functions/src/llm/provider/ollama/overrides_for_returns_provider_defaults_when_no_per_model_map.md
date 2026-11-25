---
type: Rust Function
title: overrides_for_returns_provider_defaults_when_no_per_model_map
resource: src/llm/provider/ollama.rs#L1037-L1048
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
  - target: functions/src/llm/provider/ollama/OllamaProvider/with_sampling
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/OllamaProvider/with_num_ctx
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaProvider/overrides_for
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn overrides_for_returns_provider_defaults_when_no_per_model_map()`

# Calls

- [default_local](../../../../../functions/src/llm/provider/ollama/OllamaProvider/default_local.md)
- [with_sampling](../../../../../functions/src/llm/provider/ollama/OllamaProvider/with_sampling.md)
- [with_num_ctx](../../../../../functions/src/llm/provider/ollama/OllamaProvider/with_num_ctx.md)
- [overrides_for](../../../../../functions/src/llm/provider/ollama/OllamaProvider/overrides_for.md)