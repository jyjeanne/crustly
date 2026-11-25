---
type: Rust Method
title: start_llama_cpp_switch
resource: src/tui/app.rs#L3696-L3711
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/app/App/event_sender
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/llama_cpp_download/spawn_switch
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/handle_llama_cpp_models_key
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn start_llama_cpp_switch(&mut self, path: std::path::PathBuf)`

# Calls

- [event_sender](../../../../../functions/src/tui/app/App/event_sender.md)
- [spawn_switch](../../../../../functions/src/tui/llama_cpp_download/spawn_switch.md)

# Called by

- [handle_llama_cpp_models_key](../../../../../functions/src/tui/app/App/handle_llama_cpp_models_key.md)