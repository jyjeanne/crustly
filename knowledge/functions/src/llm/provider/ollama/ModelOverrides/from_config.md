---
type: Rust Method
title: from_config
resource: src/llm/provider/ollama.rs#L84-L107
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/provider/ollama/parse_keep_alive
  called_by:
  - functions/src/llm/provider/factory/ollama_provider_from_config
  - functions/src/llm/provider/ollama/per_model_override_wins_over_provider_default_for_that_model
  - functions/src/llm/provider/ollama/per_model_override_falls_back_field_by_field
  - functions/src/llm/provider/ollama/context_window_reflects_the_per_model_num_ctx_that_is_actually_requested
  - functions/src/llm/provider/ollama/per_model_think_false_is_sent_when_request_has_no_thinking
---

# Signature

`pub fn from_config( temperature: Option<f32>, top_p: Option<f32>, top_k: Option<u32>, num_ctx: Option<u32>, keep_alive: Option<&str>, think: Option<&str>, ) -> Self`

# Calls

- [parse_keep_alive](../../../../../../functions/src/llm/provider/ollama/parse_keep_alive.md)

# Called by

- [ollama_provider_from_config](../../../../../../functions/src/llm/provider/factory/ollama_provider_from_config.md)
- [per_model_override_wins_over_provider_default_for_that_model](../../../../../../functions/src/llm/provider/ollama/per_model_override_wins_over_provider_default_for_that_model.md)
- [per_model_override_falls_back_field_by_field](../../../../../../functions/src/llm/provider/ollama/per_model_override_falls_back_field_by_field.md)
- [context_window_reflects_the_per_model_num_ctx_that_is_actually_requested](../../../../../../functions/src/llm/provider/ollama/context_window_reflects_the_per_model_num_ctx_that_is_actually_requested.md)
- [per_model_think_false_is_sent_when_request_has_no_thinking](../../../../../../functions/src/llm/provider/ollama/per_model_think_false_is_sent_when_request_has_no_thinking.md)