---
type: Rust Function
title: run_inner
resource: src/tui/runner.rs#L89-L99
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/app/App/initialize
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/event_sender
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/EventHandler/start_terminal_listener
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/runner/run_loop
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/runner/run
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn run_inner(stdout: io::Stdout, app: &mut App) -> Result<()>`

# Calls

- [initialize](../../../../functions/src/tui/app/App/initialize.md)
- [event_sender](../../../../functions/src/tui/app/App/event_sender.md)
- [start_terminal_listener](../../../../functions/src/tui/events/EventHandler/start_terminal_listener.md)
- [run_loop](../../../../functions/src/tui/runner/run_loop.md)

# Called by

- [run](../../../../functions/src/tui/runner/run.md)