---
type: Rust Method
title: open_llama_cpp_models
resource: src/tui/app.rs#L3616-L3655
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/app/App/event_sender
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/switch_mode
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/handle_key_event
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn open_llama_cpp_models(&mut self) -> Result<()>`

# Calls

- [event_sender](../../../../../functions/src/tui/app/App/event_sender.md)
- [switch_mode](../../../../../functions/src/tui/app/App/switch_mode.md)

# Called by

- [handle_key_event](../../../../../functions/src/tui/app/App/handle_key_event.md)