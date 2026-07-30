---
type: Rust Function
title: try_create_ollama
resource: src/llm/provider/factory.rs#L241-L258
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/provider/factory/ollama_provider_from_config
---

# Signature

`fn try_create_ollama(config: &Config) -> Result<Option<Arc<dyn Provider>>>`

# Calls

- [ollama_provider_from_config](../../../../../functions/src/llm/provider/factory/ollama_provider_from_config.md)