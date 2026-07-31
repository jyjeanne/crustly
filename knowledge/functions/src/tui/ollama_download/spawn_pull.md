---
type: Rust Function
title: spawn_pull
resource: src/tui/ollama_download.rs#L142-L171
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/provider/ollama_models/pull_model
---

# Signature

`pub async fn spawn_pull( host: String, model: String, event_sender: UnboundedSender<TuiEvent>, ) -> JoinHandle<()>`

# Calls

- [pull_model](../../../../functions/src/llm/provider/ollama_models/pull_model.md)