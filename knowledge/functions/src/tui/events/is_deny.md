---
type: Rust Function
title: is_deny
resource: src/tui/events.rs#L579-L584
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/tui/app/App/handle_approval_key
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn is_deny(event: &KeyEvent) -> bool`

# Called by

- [handle_approval_key](../../../../functions/src/tui/app/App/handle_approval_key.md)