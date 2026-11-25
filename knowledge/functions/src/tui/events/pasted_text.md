---
type: Rust Function
title: pasted_text
resource: src/tui/events.rs#L390-L406
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/events/typed_char
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/events/flush_run
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn pasted_text(keys: &[KeyEvent]) -> String`

# Calls

- [typed_char](../../../../functions/src/tui/events/typed_char.md)

# Called by

- [flush_run](../../../../functions/src/tui/events/flush_run.md)