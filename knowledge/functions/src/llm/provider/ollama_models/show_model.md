---
type: Rust Function
title: show_model
resource: src/llm/provider/ollama_models.rs#L86-L99
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/ollama_models/client_for
  - functions/src/tui/error/ErrorInfo/with_context
  called_by:
  - functions/src/cli/cmd_ollama
  - functions/src/llm/provider/ollama_models/show_model_parses_minimal_response
---

# Signature

`pub async fn show_model(host: &str, model_name: &str) -> Result<ModelDetails>`

# Calls

- [client_for](../../../../../functions/src/llm/provider/ollama_models/client_for.md)
- [with_context](../../../../../functions/src/tui/error/ErrorInfo/with_context.md)

# Called by

- [cmd_ollama](../../../../../functions/src/cli/cmd_ollama.md)
- [show_model_parses_minimal_response](../../../../../functions/src/llm/provider/ollama_models/show_model_parses_minimal_response.md)