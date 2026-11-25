---
type: Rust Method
title: initialize
resource: src/tui/app.rs#L781-L794
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/services/session/SessionService/get_most_recent_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/load_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/create_new_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/load_sessions
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/runner/run_inner
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn initialize(&mut self) -> Result<()>`

# Calls

- [get_most_recent_session](../../../../../functions/src/services/session/SessionService/get_most_recent_session.md)
- [load_session](../../../../../functions/src/tui/app/App/load_session.md)
- [create_new_session](../../../../../functions/src/tui/app/App/create_new_session.md)
- [load_sessions](../../../../../functions/src/tui/app/App/load_sessions.md)

# Called by

- [run_inner](../../../../../functions/src/tui/runner/run_inner.md)