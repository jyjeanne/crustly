---
type: Rust Method
title: with_per_model
resource: src/llm/provider/ollama.rs#L192-L198
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/provider/factory/ollama_provider_from_config
  - functions/src/llm/provider/ollama/per_model_override_wins_over_provider_default_for_that_model
  - functions/src/llm/provider/ollama/per_model_override_falls_back_field_by_field
  - functions/src/llm/provider/ollama/context_window_reflects_the_per_model_num_ctx_that_is_actually_requested
  - functions/src/llm/provider/ollama/per_model_think_false_is_sent_when_request_has_no_thinking
---

# Signature

`pub fn with_per_model( mut self, per_model: std::collections::HashMap<String, ModelOverrides>, ) -> Self`

# Called by

- [ollama_provider_from_config](../../../../../../functions/src/llm/provider/factory/ollama_provider_from_config.md)
- [per_model_override_wins_over_provider_default_for_that_model](../../../../../../functions/src/llm/provider/ollama/per_model_override_wins_over_provider_default_for_that_model.md)
- [per_model_override_falls_back_field_by_field](../../../../../../functions/src/llm/provider/ollama/per_model_override_falls_back_field_by_field.md)
- [context_window_reflects_the_per_model_num_ctx_that_is_actually_requested](../../../../../../functions/src/llm/provider/ollama/context_window_reflects_the_per_model_num_ctx_that_is_actually_requested.md)
- [per_model_think_false_is_sent_when_request_has_no_thinking](../../../../../../functions/src/llm/provider/ollama/per_model_think_false_is_sent_when_request_has_no_thinking.md)