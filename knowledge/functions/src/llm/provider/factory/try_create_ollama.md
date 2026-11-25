---
type: Rust Function
title: try_create_ollama
resource: src/llm/provider/factory.rs#L288-L303
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/factory/ollama_provider_from_config
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn try_create_ollama(config: &Config) -> Result<Option<Arc<dyn Provider>>>`

# Calls

- [ollama_provider_from_config](../../../../../functions/src/llm/provider/factory/ollama_provider_from_config.md)