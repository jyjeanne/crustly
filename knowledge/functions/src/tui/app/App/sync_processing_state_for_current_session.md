---
type: Rust Method
title: sync_processing_state_for_current_session
resource: src/tui/app.rs#L2511-L2517
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/tui/app/App/create_new_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/load_session
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn sync_processing_state_for_current_session(&mut self)`

# Called by

- [create_new_session](../../../../../functions/src/tui/app/App/create_new_session.md)
- [load_session](../../../../../functions/src/tui/app/App/load_session.md)