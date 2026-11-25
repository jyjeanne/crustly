---
type: Rust Function
title: key_release_events_are_ignored
resource: src/tui/app.rs#L4275-L4303
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/app/App/push_input_history
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/input_text
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn key_release_events_are_ignored()`

# Calls

- [push_input_history](../../../../functions/src/tui/app/App/push_input_history.md)
- [handle_event](../../../../functions/src/tui/app/App/handle_event.md)
- [input_text](../../../../functions/src/tui/app/App/input_text.md)