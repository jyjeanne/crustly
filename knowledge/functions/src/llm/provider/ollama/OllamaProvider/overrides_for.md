---
type: Rust Method
title: overrides_for
resource: src/llm/provider/ollama.rs#L213-L242
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/ollama/OllamaProvider/to_ollama_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaProvider/provider/context_window
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/per_model_override_wins_over_provider_default_for_that_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/per_model_override_falls_back_field_by_field
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/overrides_for_returns_provider_defaults_when_no_per_model_map
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn overrides_for(&self, model: &str) -> ModelOverrides`

# Called by

- [to_ollama_request](../../../../../../functions/src/llm/provider/ollama/OllamaProvider/to_ollama_request.md)
- [context_window](../../../../../../functions/src/llm/provider/ollama/OllamaProvider/provider/context_window.md)
- [per_model_override_wins_over_provider_default_for_that_model](../../../../../../functions/src/llm/provider/ollama/per_model_override_wins_over_provider_default_for_that_model.md)
- [per_model_override_falls_back_field_by_field](../../../../../../functions/src/llm/provider/ollama/per_model_override_falls_back_field_by_field.md)
- [overrides_for_returns_provider_defaults_when_no_per_model_map](../../../../../../functions/src/llm/provider/ollama/overrides_for_returns_provider_defaults_when_no_per_model_map.md)