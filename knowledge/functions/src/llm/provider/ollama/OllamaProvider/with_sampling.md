---
type: Rust Method
title: with_sampling
resource: src/llm/provider/ollama.rs#L269-L279
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/factory/ollama_provider_from_config
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/per_model_override_wins_over_provider_default_for_that_model
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/per_model_override_falls_back_field_by_field
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/overrides_for_returns_provider_defaults_when_no_per_model_map
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn with_sampling( mut self, temperature: Option<f32>, top_p: Option<f32>, top_k: Option<u32>, ) -> Self`

# Called by

- [ollama_provider_from_config](../../../../../../functions/src/llm/provider/factory/ollama_provider_from_config.md)
- [per_model_override_wins_over_provider_default_for_that_model](../../../../../../functions/src/llm/provider/ollama/per_model_override_wins_over_provider_default_for_that_model.md)
- [per_model_override_falls_back_field_by_field](../../../../../../functions/src/llm/provider/ollama/per_model_override_falls_back_field_by_field.md)
- [overrides_for_returns_provider_defaults_when_no_per_model_map](../../../../../../functions/src/llm/provider/ollama/overrides_for_returns_provider_defaults_when_no_per_model_map.md)