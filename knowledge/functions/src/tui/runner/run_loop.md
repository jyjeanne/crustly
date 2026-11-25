---
type: Rust Function
title: run_loop
resource: src/tui/runner.rs#L102-L150
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/render/render
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/next_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/try_next_event
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/runner/run_inner
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/runner/run_loop_exits_immediately_when_should_quit_is_set
    resolved_by: tree-sitter
    confidence: exact
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