---
type: Rust Method
title: initialize
resource: src/tui/app.rs#L432-L445
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/services/session/SessionService/get_most_recent_session
  - functions/src/tui/app/App/load_session
  - functions/src/tui/app/App/create_new_session
  - functions/src/tui/app/App/load_sessions
  called_by:
  - functions/src/tui/runner/run_inner
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