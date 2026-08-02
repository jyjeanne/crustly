---
type: Rust Function
title: pull_model
resource: src/llm/provider/ollama_models.rs#L116-L140
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/ollama_models/client_for
  - functions/src/tui/error/ErrorInfo/with_context
  - functions/src/tui/events/EventHandler/next
  called_by:
  - functions/src/cli/cmd_ollama
  - functions/src/llm/provider/ollama_models/pull_model_forwards_progress_and_completes
  - functions/src/tui/ollama_download/spawn_pull
---

# Signature

`pub async fn pull_model( host: &str, model_name: &str, progress_tx: UnboundedSender<PullProgress>, ) -> Result<()>`

# Calls

- [client_for](../../../../../functions/src/llm/provider/ollama_models/client_for.md)
- [with_context](../../../../../functions/src/tui/error/ErrorInfo/with_context.md)
- [next](../../../../../functions/src/tui/events/EventHandler/next.md)

# Called by

- [cmd_ollama](../../../../../functions/src/cli/cmd_ollama.md)
- [pull_model_forwards_progress_and_completes](../../../../../functions/src/llm/provider/ollama_models/pull_model_forwards_progress_and_completes.md)
- [spawn_pull](../../../../../functions/src/tui/ollama_download/spawn_pull.md)