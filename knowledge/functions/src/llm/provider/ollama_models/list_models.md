---
type: Rust Function
title: list_models
resource: src/llm/provider/ollama_models.rs#L68-L83
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/ollama_models/client_for
  - functions/src/llm/provider/llama_cpp_models/list_local_models
  called_by:
  - functions/src/cli/cmd_ollama
  - functions/src/llm/provider/ollama_models/list_models_parses_tags_response
  - functions/src/tui/ollama_download/fetch_installed_models
---

# Signature

`pub async fn list_models(host: &str) -> Result<Vec<LocalModelInfo>>`

# Calls

- [client_for](../../../../../functions/src/llm/provider/ollama_models/client_for.md)
- [list_local_models](../../../../../functions/src/llm/provider/llama_cpp_models/list_local_models.md)

# Called by

- [cmd_ollama](../../../../../functions/src/cli/cmd_ollama.md)
- [list_models_parses_tags_response](../../../../../functions/src/llm/provider/ollama_models/list_models_parses_tags_response.md)
- [fetch_installed_models](../../../../../functions/src/tui/ollama_download/fetch_installed_models.md)