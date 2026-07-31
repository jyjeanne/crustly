---
type: Rust Function
title: cmd_ollama
resource: src/cli/mod.rs#L1103-L1194
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/cli/ollama_host
  - functions/src/llm/provider/ollama_models/list_models
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/llm/provider/ollama_models/pull_model
  - functions/src/tui/error/ErrorInfo/with_context
  - functions/src/llm/provider/ollama_models/delete_model
  - functions/src/llm/provider/ollama_models/show_model
  - functions/src/llm/provider/ollama_models/generate_embeddings
  - functions/src/tui/events/EventHandler/next
---

# Signature

`async fn cmd_ollama(config: &crate::config::Config, operation: OllamaCommands) -> Result<()>`

# Calls

- [ollama_host](../../../functions/src/cli/ollama_host.md)
- [list_models](../../../functions/src/llm/provider/ollama_models/list_models.md)
- [is_empty](../../../functions/src/config/secrets/SecretString/is_empty.md)
- [pull_model](../../../functions/src/llm/provider/ollama_models/pull_model.md)
- [with_context](../../../functions/src/tui/error/ErrorInfo/with_context.md)
- [delete_model](../../../functions/src/llm/provider/ollama_models/delete_model.md)
- [show_model](../../../functions/src/llm/provider/ollama_models/show_model.md)
- [generate_embeddings](../../../functions/src/llm/provider/ollama_models/generate_embeddings.md)
- [next](../../../functions/src/tui/events/EventHandler/next.md)