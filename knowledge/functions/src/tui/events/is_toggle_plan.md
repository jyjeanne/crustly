---
type: Rust Function
title: is_toggle_plan
resource: src/tui/events.rs#L474-L476
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/events/key_matches
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/handle_key_event
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn is_toggle_plan(event: &KeyEvent) -> bool`

# Calls

- [key_matches](../../../../functions/src/tui/events/key_matches.md)

# Called by

- [handle_key_event](../../../../functions/src/tui/app/App/handle_key_event.md)