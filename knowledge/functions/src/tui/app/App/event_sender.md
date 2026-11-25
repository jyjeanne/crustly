---
type: Rust Method
title: event_sender
resource: src/tui/app.rs#L807-L809
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/events/EventHandler/sender
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_chat
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/send_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_approval_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/open_model_download
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/start_model_pull
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/start_model_delete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/open_provider_switch
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/open_llama_cpp_models
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/start_llama_cpp_download
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/start_llama_cpp_delete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/start_llama_cpp_switch
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/runner/run_inner
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn event_sender(&self) -> tokio::sync::mpsc::UnboundedSender<TuiEvent>`

# Calls

- [sender](../../../../../functions/src/tui/events/EventHandler/sender.md)

# Called by

- [cmd_chat](../../../../../functions/src/cli/cmd_chat.md)
- [handle_event](../../../../../functions/src/tui/app/App/handle_event.md)
- [send_message](../../../../../functions/src/tui/app/App/send_message.md)
- [handle_approval_key](../../../../../functions/src/tui/app/App/handle_approval_key.md)
- [open_model_download](../../../../../functions/src/tui/app/App/open_model_download.md)
- [start_model_pull](../../../../../functions/src/tui/app/App/start_model_pull.md)
- [start_model_delete](../../../../../functions/src/tui/app/App/start_model_delete.md)
- [open_provider_switch](../../../../../functions/src/tui/app/App/open_provider_switch.md)
- [open_llama_cpp_models](../../../../../functions/src/tui/app/App/open_llama_cpp_models.md)
- [start_llama_cpp_download](../../../../../functions/src/tui/app/App/start_llama_cpp_download.md)
- [start_llama_cpp_delete](../../../../../functions/src/tui/app/App/start_llama_cpp_delete.md)
- [start_llama_cpp_switch](../../../../../functions/src/tui/app/App/start_llama_cpp_switch.md)
- [run_inner](../../../../../functions/src/tui/runner/run_inner.md)