---
type: Rust Function
title: list_models
resource: src/llm/provider/ollama_models.rs#L68-L83
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/ollama_models/client_for
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_ollama
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama_models/list_models_parses_tags_response
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/ollama_download/fetch_installed_models
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn list_models(host: &str) -> Result<Vec<LocalModelInfo>>`

# Calls

- [client_for](../../../../../functions/src/llm/provider/ollama_models/client_for.md)
- [context](../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [cmd_ollama](../../../../../functions/src/cli/cmd_ollama.md)
- [list_models_parses_tags_response](../../../../../functions/src/llm/provider/ollama_models/list_models_parses_tags_response.md)
- [fetch_installed_models](../../../../../functions/src/tui/ollama_download/fetch_installed_models.md)