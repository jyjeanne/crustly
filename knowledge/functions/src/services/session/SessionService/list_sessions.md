---
type: Rust Method
title: list_sessions
resource: src/services/session.rs#L66-L69
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/db/repository/session/SessionRepository/list
  called_by:
  - functions/src/services/session/test_list_sessions
  - functions/src/tui/app/App/load_sessions
  - functions/tests/integration_test/test_end_to_end_session_management
---

# Signature

`pub async fn list_sessions(&self, options: SessionListOptions) -> Result<Vec<Session>>`

# Calls

- [list](../../../../../functions/src/db/repository/session/SessionRepository/list.md)

# Called by

- [test_list_sessions](../../../../../functions/src/services/session/test_list_sessions.md)
- [load_sessions](../../../../../functions/src/tui/app/App/load_sessions.md)
- [test_end_to_end_session_management](../../../../../functions/tests/integration_test/test_end_to_end_session_management.md)