---
type: Rust Function
title: flush_run
resource: src/tui/events.rs#L379-L386
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/events/pasted_text
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/events/coalesce_input
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn flush_run(run: &mut Vec<KeyEvent>, events: &mut Vec<TuiEvent>)`

# Calls

- [pasted_text](../../../../functions/src/tui/events/pasted_text.md)

# Called by

- [coalesce_input](../../../../functions/src/tui/events/coalesce_input.md)