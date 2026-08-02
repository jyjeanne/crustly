---
type: Rust Method
title: try_next_event
resource: src/tui/app.rs#L628-L630
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/tui/events/EventHandler/try_next
  called_by:
  - functions/src/tui/runner/run_loop
---

# Signature

`pub fn try_next_event(&mut self) -> Option<TuiEvent>`

# Calls

- [try_next](../../../../../functions/src/tui/events/EventHandler/try_next.md)

# Called by

- [run_loop](../../../../../functions/src/tui/runner/run_loop.md)