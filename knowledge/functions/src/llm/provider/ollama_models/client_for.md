---
type: Rust Function
title: client_for
resource: src/llm/provider/ollama_models.rs#L63-L65
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/ollama_models/list_models
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama_models/show_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama_models/delete_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama_models/pull_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama_models/generate_embeddings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama_models/invalid_host_returns_error
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn client_for(host: &str) -> Result<Ollama>`

# Called by

- [list_models](../../../../../functions/src/llm/provider/ollama_models/list_models.md)
- [show_model](../../../../../functions/src/llm/provider/ollama_models/show_model.md)
- [delete_model](../../../../../functions/src/llm/provider/ollama_models/delete_model.md)
- [pull_model](../../../../../functions/src/llm/provider/ollama_models/pull_model.md)
- [generate_embeddings](../../../../../functions/src/llm/provider/ollama_models/generate_embeddings.md)
- [invalid_host_returns_error](../../../../../functions/src/llm/provider/ollama_models/invalid_host_returns_error.md)