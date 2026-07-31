---
type: Rust Method
title: next_event
resource: src/tui/app.rs#L519-L521
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/tui/events/EventHandler/next
  called_by:
  - functions/src/tui/runner/run_loop
---

# Signature

`pub async fn next_event(&mut self) -> Option<TuiEvent>`

# Calls

- [next](../../../../../functions/src/tui/events/EventHandler/next.md)

# Called by

- [run_loop](../../../../../functions/src/tui/runner/run_loop.md)