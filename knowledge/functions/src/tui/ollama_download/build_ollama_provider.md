---
type: Rust Function
title: build_ollama_provider
resource: src/tui/ollama_download.rs#L104-L123
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/factory/ollama_provider_from_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaProvider/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/OllamaProvider/with_default_model
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn build_ollama_provider( host: &str, model: &str, config: Option<&crate::config::OllamaProviderConfig>, ) -> Result<std::sync::Arc<dyn crate::llm::provider::Provider>, String>`

# Calls

- [ollama_provider_from_config](../../../../functions/src/llm/provider/factory/ollama_provider_from_config.md)
- [new](../../../../functions/src/llm/provider/ollama/OllamaProvider/new.md)
- [with_default_model](../../../../functions/src/llm/provider/ollama/OllamaProvider/with_default_model.md)