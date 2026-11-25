---
type: Rust Function
title: spawn_switch
resource: src/tui/llama_cpp_download.rs#L274-L301
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/tui/app/App/start_llama_cpp_switch
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn spawn_switch( model_path: PathBuf, config: Option<crate::config::LlamaCppProviderConfig>, slot: PendingProvider, event_sender: UnboundedSender<TuiEvent>, ) -> JoinHandle<()>`

# Called by

- [start_llama_cpp_switch](../../../../functions/src/tui/app/App/start_llama_cpp_switch.md)