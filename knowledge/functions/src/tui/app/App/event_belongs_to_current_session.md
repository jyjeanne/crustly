---
type: Rust Method
title: event_belongs_to_current_session
resource: src/tui/app.rs#L1381-L1385
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/tui/app/App/handle_event
---

# Signature

`fn event_belongs_to_current_session(&self, session_id: Uuid) -> bool`

# Called by

- [handle_event](../../../../../functions/src/tui/app/App/handle_event.md)