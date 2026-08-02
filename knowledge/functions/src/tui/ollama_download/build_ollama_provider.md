---
type: Rust Function
title: build_ollama_provider
resource: src/tui/ollama_download.rs#L104-L123
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/factory/ollama_provider_from_config
---

# Signature

`pub fn build_ollama_provider( host: &str, model: &str, config: Option<&crate::config::OllamaProviderConfig>, ) -> Result<std::sync::Arc<dyn crate::llm::provider::Provider>, String>`

# Calls

- [ollama_provider_from_config](../../../../functions/src/llm/provider/factory/ollama_provider_from_config.md)