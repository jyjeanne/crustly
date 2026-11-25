---
type: Rust Method
title: next_event
resource: src/tui/app.rs#L991-L993
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/events/EventHandler/next
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/runner/run_loop
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn next_event(&mut self) -> Option<TuiEvent>`

# Calls

- [next](../../../../../functions/src/tui/events/EventHandler/next.md)

# Called by

- [run_loop](../../../../../functions/src/tui/runner/run_loop.md)