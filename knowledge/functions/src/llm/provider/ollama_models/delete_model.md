---
type: Rust Function
title: delete_model
resource: src/llm/provider/ollama_models.rs#L102-L109
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/provider/ollama_models/client_for
  - functions/src/tui/error/ErrorInfo/with_context
  called_by:
  - functions/src/cli/cmd_ollama
  - functions/src/llm/provider/ollama_models/delete_model_succeeds_on_2xx
  - functions/src/tui/ollama_download/spawn_delete
---

# Signature

`pub async fn delete_model(host: &str, model_name: &str) -> Result<()>`

# Calls

- [client_for](../../../../../functions/src/llm/provider/ollama_models/client_for.md)
- [with_context](../../../../../functions/src/tui/error/ErrorInfo/with_context.md)

# Called by

- [cmd_ollama](../../../../../functions/src/cli/cmd_ollama.md)
- [delete_model_succeeds_on_2xx](../../../../../functions/src/llm/provider/ollama_models/delete_model_succeeds_on_2xx.md)
- [spawn_delete](../../../../../functions/src/tui/ollama_download/spawn_delete.md)