---
type: Rust Method
title: overrides_for
resource: src/llm/provider/ollama.rs#L213-L242
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/llm/provider/ollama/OllamaProvider/to_ollama_request
  - functions/src/llm/provider/ollama/OllamaProvider/provider/context_window
  - functions/src/llm/provider/ollama/per_model_override_wins_over_provider_default_for_that_model
  - functions/src/llm/provider/ollama/per_model_override_falls_back_field_by_field
  - functions/src/llm/provider/ollama/overrides_for_returns_provider_defaults_when_no_per_model_map
---

# Signature

`fn overrides_for(&self, model: &str) -> ModelOverrides`

# Calls

- [is_empty](../../../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [to_ollama_request](../../../../../../functions/src/llm/provider/ollama/OllamaProvider/to_ollama_request.md)
- [context_window](../../../../../../functions/src/llm/provider/ollama/OllamaProvider/provider/context_window.md)
- [per_model_override_wins_over_provider_default_for_that_model](../../../../../../functions/src/llm/provider/ollama/per_model_override_wins_over_provider_default_for_that_model.md)
- [per_model_override_falls_back_field_by_field](../../../../../../functions/src/llm/provider/ollama/per_model_override_falls_back_field_by_field.md)
- [overrides_for_returns_provider_defaults_when_no_per_model_map](../../../../../../functions/src/llm/provider/ollama/overrides_for_returns_provider_defaults_when_no_per_model_map.md)