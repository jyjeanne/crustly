---
type: Rust Method
title: load_sessions
resource: src/tui/app.rs#L1232-L1245
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/services/session/SessionService/list_sessions
  called_by:
  - functions/src/tui/app/App/initialize
  - functions/src/tui/app/App/create_new_session
  - functions/src/tui/app/App/switch_mode
---

# Signature

`async fn load_sessions(&mut self) -> Result<()>`

# Calls

- [list_sessions](../../../../../functions/src/services/session/SessionService/list_sessions.md)

# Called by

- [initialize](../../../../../functions/src/tui/app/App/initialize.md)
- [create_new_session](../../../../../functions/src/tui/app/App/create_new_session.md)
- [switch_mode](../../../../../functions/src/tui/app/App/switch_mode.md)