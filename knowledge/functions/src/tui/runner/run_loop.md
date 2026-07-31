---
type: Rust Function
title: run_loop
resource: src/tui/runner.rs#L97-L145
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/tui/render/render
  - functions/src/tui/app/App/next_event
  - functions/src/tui/app/App/handle_event
  - functions/src/tui/app/App/try_next_event
  called_by:
  - functions/src/tui/runner/run_inner
  - functions/src/tui/runner/run_loop_exits_immediately_when_should_quit_is_set
---

# Signature

`async fn run_loop<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> where <B as Backend>::Error: std::error::Error + Send + Sync + 'static,`

# Calls

- [render](../../../../functions/src/tui/render/render.md)
- [next_event](../../../../functions/src/tui/app/App/next_event.md)
- [handle_event](../../../../functions/src/tui/app/App/handle_event.md)
- [try_next_event](../../../../functions/src/tui/app/App/try_next_event.md)

# Called by

- [run_inner](../../../../functions/src/tui/runner/run_inner.md)
- [run_loop_exits_immediately_when_should_quit_is_set](../../../../functions/src/tui/runner/run_loop_exits_immediately_when_should_quit_is_set.md)