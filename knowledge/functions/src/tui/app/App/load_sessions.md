---
type: Rust Method
title: load_sessions
resource: src/tui/app.rs#L2326-L2339
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/services/session/SessionService/list_sessions
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/initialize
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/create_new_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/switch_mode
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn load_sessions(&mut self) -> Result<()>`

# Calls

- [list_sessions](../../../../../functions/src/services/session/SessionService/list_sessions.md)

# Called by

- [initialize](../../../../../functions/src/tui/app/App/initialize.md)
- [create_new_session](../../../../../functions/src/tui/app/App/create_new_session.md)
- [switch_mode](../../../../../functions/src/tui/app/App/switch_mode.md)