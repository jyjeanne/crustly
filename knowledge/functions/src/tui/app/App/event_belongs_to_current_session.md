---
type: Rust Method
title: event_belongs_to_current_session
resource: src/tui/app.rs#L2486-L2490
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/tui/app/App/handle_event
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn event_belongs_to_current_session(&self, session_id: Uuid) -> bool`

# Called by

- [handle_event](../../../../../functions/src/tui/app/App/handle_event.md)