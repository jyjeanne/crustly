---
type: Rust Method
title: event_sender
resource: src/tui/app.rs#L458-L460
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/tui/events/EventHandler/sender
  called_by:
  - functions/src/cli/cmd_chat
  - functions/src/tui/app/App/handle_event
  - functions/src/tui/app/App/send_message
  - functions/src/tui/app/App/handle_approval_key
  - functions/src/tui/app/App/open_model_download
  - functions/src/tui/app/App/start_model_pull
  - functions/src/tui/app/App/start_model_delete
  - functions/src/tui/app/App/open_provider_switch
  - functions/src/tui/runner/run_inner
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
- [run_inner](../../../../../functions/src/tui/runner/run_inner.md)