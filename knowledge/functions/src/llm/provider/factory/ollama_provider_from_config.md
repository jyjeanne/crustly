---
type: Rust Function
title: ollama_provider_from_config
resource: src/llm/provider/factory.rs#L317-L368
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/ollama/OllamaProvider/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/OllamaProvider/with_default_model
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/OllamaProvider/with_keep_alive
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaProvider/with_num_ctx
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaProvider/with_sampling
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/OllamaProvider/with_think
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/ModelOverrides/from_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaProvider/with_per_model
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/factory/try_create_ollama
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/ollama_download/build_ollama_provider
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn ollama_provider_from_config( cfg: &crate::config::OllamaProviderConfig, model_override: Option<&str>, ) -> super::ollama::OllamaProvider`

# Calls

- [new](../../../../../functions/src/llm/provider/ollama/OllamaProvider/new.md)
- [with_default_model](../../../../../functions/src/llm/provider/ollama/OllamaProvider/with_default_model.md)
- [with_keep_alive](../../../../../functions/src/llm/provider/ollama/OllamaProvider/with_keep_alive.md)
- [with_num_ctx](../../../../../functions/src/llm/provider/ollama/OllamaProvider/with_num_ctx.md)
- [with_sampling](../../../../../functions/src/llm/provider/ollama/OllamaProvider/with_sampling.md)
- [with_think](../../../../../functions/src/llm/provider/ollama/OllamaProvider/with_think.md)
- [from_config](../../../../../functions/src/llm/provider/ollama/ModelOverrides/from_config.md)
- [with_per_model](../../../../../functions/src/llm/provider/ollama/OllamaProvider/with_per_model.md)

# Called by

- [try_create_ollama](../../../../../functions/src/llm/provider/factory/try_create_ollama.md)
- [build_ollama_provider](../../../../../functions/src/tui/ollama_download/build_ollama_provider.md)