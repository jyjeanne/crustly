---
type: Rust Function
title: context_window_reflects_the_per_model_num_ctx_that_is_actually_requested
resource: src/llm/provider/ollama.rs#L1114-L1138
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/provider/ollama/ModelOverrides/from_config
  - functions/src/llm/provider/ollama/OllamaProvider/default_local
  - functions/src/llm/provider/ollama/OllamaProvider/with_num_ctx
  - functions/src/llm/provider/ollama/OllamaProvider/with_per_model
---

# Signature

`fn context_window_reflects_the_per_model_num_ctx_that_is_actually_requested()`

# Calls

- [from_config](../../../../../functions/src/llm/provider/ollama/ModelOverrides/from_config.md)
- [default_local](../../../../../functions/src/llm/provider/ollama/OllamaProvider/default_local.md)
- [with_num_ctx](../../../../../functions/src/llm/provider/ollama/OllamaProvider/with_num_ctx.md)
- [with_per_model](../../../../../functions/src/llm/provider/ollama/OllamaProvider/with_per_model.md)