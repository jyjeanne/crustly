---
type: Rust Method
title: start_terminal_listener
resource: src/tui/events.rs#L216-L254
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/tui/runner/run_inner
---

# Signature

`pub fn start_terminal_listener(tx: mpsc::UnboundedSender<TuiEvent>)`

# Called by

- [run_inner](../../../../../functions/src/tui/runner/run_inner.md)