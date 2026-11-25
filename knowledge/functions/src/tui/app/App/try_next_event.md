---
type: Rust Method
title: try_next_event
resource: src/tui/app.rs#L996-L998
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/events/EventHandler/try_next
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/runner/run_loop
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn try_next_event(&mut self) -> Option<TuiEvent>`

# Calls

- [try_next](../../../../../functions/src/tui/events/EventHandler/try_next.md)

# Called by

- [run_loop](../../../../../functions/src/tui/runner/run_loop.md)