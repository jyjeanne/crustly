---
type: Rust Function
title: spawn_delete
resource: src/tui/ollama_download.rs#L196-L210
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/provider/ollama_models/delete_model
---

# Signature

`pub async fn spawn_delete( host: String, model: String, event_sender: UnboundedSender<TuiEvent>, ) -> JoinHandle<()>`

# Calls

- [delete_model](../../../../functions/src/llm/provider/ollama_models/delete_model.md)