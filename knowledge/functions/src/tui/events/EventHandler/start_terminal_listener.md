---
type: Rust Method
title: start_terminal_listener
resource: src/tui/events.rs#L248-L286
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/tui/runner/run_inner
---

# Signature

`pub fn start_terminal_listener(tx: mpsc::UnboundedSender<TuiEvent>)`

# Called by

- [run_inner](../../../../../functions/src/tui/runner/run_inner.md)