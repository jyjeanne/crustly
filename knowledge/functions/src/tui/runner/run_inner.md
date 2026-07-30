---
type: Rust Function
title: run_inner
resource: src/tui/runner.rs#L84-L94
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/tui/app/App/initialize
  - functions/src/tui/app/App/event_sender
  - functions/src/tui/events/EventHandler/start_terminal_listener
  - functions/src/tui/runner/run_loop
  called_by:
  - functions/src/tui/runner/run
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