---
type: Rust Function
title: spawn_pull
resource: src/tui/ollama_download.rs#L142-L171
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/ollama_models/pull_model
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn spawn_pull( host: String, model: String, event_sender: UnboundedSender<TuiEvent>, ) -> JoinHandle<()>`

# Calls

- [pull_model](../../../../functions/src/llm/provider/ollama_models/pull_model.md)