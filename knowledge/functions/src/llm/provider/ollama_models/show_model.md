---
type: Rust Function
title: show_model
resource: src/llm/provider/ollama_models.rs#L86-L99
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/ollama_models/client_for
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/error/ErrorInfo/with_context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_ollama
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama_models/show_model_parses_minimal_response
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn show_model(host: &str, model_name: &str) -> Result<ModelDetails>`

# Calls

- [client_for](../../../../../functions/src/llm/provider/ollama_models/client_for.md)
- [with_context](../../../../../functions/src/tui/error/ErrorInfo/with_context.md)

# Called by

- [cmd_ollama](../../../../../functions/src/cli/cmd_ollama.md)
- [show_model_parses_minimal_response](../../../../../functions/src/llm/provider/ollama_models/show_model_parses_minimal_response.md)