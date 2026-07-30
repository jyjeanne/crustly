---
type: Rust Function
title: ollama_provider_from_config
resource: src/llm/provider/factory.rs#L272-L323
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/provider/ollama/OllamaProvider/with_keep_alive
  - functions/src/llm/provider/ollama/OllamaProvider/with_num_ctx
  - functions/src/llm/provider/ollama/OllamaProvider/with_think
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/llm/provider/ollama/ModelOverrides/from_config
  - functions/src/llm/provider/ollama/OllamaProvider/with_per_model
  called_by:
  - functions/src/llm/provider/factory/try_create_ollama
  - functions/src/tui/ollama_download/build_ollama_provider
---

# Signature

`pub fn ollama_provider_from_config( cfg: &crate::config::OllamaProviderConfig, model_override: Option<&str>, ) -> super::ollama::OllamaProvider`

# Calls

- [with_keep_alive](../../../../../functions/src/llm/provider/ollama/OllamaProvider/with_keep_alive.md)
- [with_num_ctx](../../../../../functions/src/llm/provider/ollama/OllamaProvider/with_num_ctx.md)
- [with_think](../../../../../functions/src/llm/provider/ollama/OllamaProvider/with_think.md)
- [is_empty](../../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [from_config](../../../../../functions/src/llm/provider/ollama/ModelOverrides/from_config.md)
- [with_per_model](../../../../../functions/src/llm/provider/ollama/OllamaProvider/with_per_model.md)

# Called by

- [try_create_ollama](../../../../../functions/src/llm/provider/factory/try_create_ollama.md)
- [build_ollama_provider](../../../../../functions/src/tui/ollama_download/build_ollama_provider.md)