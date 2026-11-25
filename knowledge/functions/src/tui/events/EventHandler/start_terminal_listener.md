---
type: Rust Method
title: start_terminal_listener
resource: src/tui/events.rs#L265-L272
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/events/read_terminal_events
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/runner/run_inner
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn start_terminal_listener(tx: mpsc::UnboundedSender<TuiEvent>)`

# Calls

- [read_terminal_events](../../../../../functions/src/tui/events/read_terminal_events.md)

# Called by

- [run_inner](../../../../../functions/src/tui/runner/run_inner.md)