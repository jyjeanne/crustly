---
type: Rust Function
title: client_for
resource: src/llm/provider/ollama_models.rs#L63-L65
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/tui/error/ErrorInfo/with_context
  called_by:
  - functions/src/llm/provider/ollama_models/list_models
  - functions/src/llm/provider/ollama_models/show_model
  - functions/src/llm/provider/ollama_models/delete_model
  - functions/src/llm/provider/ollama_models/pull_model
  - functions/src/llm/provider/ollama_models/generate_embeddings
  - functions/src/llm/provider/ollama_models/invalid_host_returns_error
---

# Signature

`fn client_for(host: &str) -> Result<Ollama>`

# Calls

- [with_context](../../../../../functions/src/tui/error/ErrorInfo/with_context.md)

# Called by

- [list_models](../../../../../functions/src/llm/provider/ollama_models/list_models.md)
- [show_model](../../../../../functions/src/llm/provider/ollama_models/show_model.md)
- [delete_model](../../../../../functions/src/llm/provider/ollama_models/delete_model.md)
- [pull_model](../../../../../functions/src/llm/provider/ollama_models/pull_model.md)
- [generate_embeddings](../../../../../functions/src/llm/provider/ollama_models/generate_embeddings.md)
- [invalid_host_returns_error](../../../../../functions/src/llm/provider/ollama_models/invalid_host_returns_error.md)