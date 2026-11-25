---
type: Rust Function
title: per_model_override_wins_over_provider_default_for_that_model
resource: src/llm/provider/ollama.rs#L994-L1016
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/ollama/ModelOverrides/from_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaProvider/default_local
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaProvider/with_sampling
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/OllamaProvider/with_per_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaProvider/overrides_for
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn per_model_override_wins_over_provider_default_for_that_model()`

# Calls

- [from_config](../../../../../functions/src/llm/provider/ollama/ModelOverrides/from_config.md)
- [default_local](../../../../../functions/src/llm/provider/ollama/OllamaProvider/default_local.md)
- [with_sampling](../../../../../functions/src/llm/provider/ollama/OllamaProvider/with_sampling.md)
- [with_per_model](../../../../../functions/src/llm/provider/ollama/OllamaProvider/with_per_model.md)
- [overrides_for](../../../../../functions/src/llm/provider/ollama/OllamaProvider/overrides_for.md)