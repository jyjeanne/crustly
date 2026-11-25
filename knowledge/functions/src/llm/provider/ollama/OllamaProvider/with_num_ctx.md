---
type: Rust Method
title: with_num_ctx
resource: src/llm/provider/ollama.rs#L263-L266
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/factory/ollama_provider_from_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/overrides_for_returns_provider_defaults_when_no_per_model_map
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/context_window_reflects_the_per_model_num_ctx_that_is_actually_requested
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/test_context_window_default_and_custom
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn with_num_ctx(mut self, num_ctx: u32) -> Self`

# Called by

- [ollama_provider_from_config](../../../../../../functions/src/llm/provider/factory/ollama_provider_from_config.md)
- [overrides_for_returns_provider_defaults_when_no_per_model_map](../../../../../../functions/src/llm/provider/ollama/overrides_for_returns_provider_defaults_when_no_per_model_map.md)
- [context_window_reflects_the_per_model_num_ctx_that_is_actually_requested](../../../../../../functions/src/llm/provider/ollama/context_window_reflects_the_per_model_num_ctx_that_is_actually_requested.md)
- [test_context_window_default_and_custom](../../../../../../functions/src/llm/provider/ollama/test_context_window_default_and_custom.md)