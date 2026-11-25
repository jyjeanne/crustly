---
type: Rust Method
title: list_sessions
resource: src/services/session.rs#L64-L67
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/repository/session/SessionRepository/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceContext/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/session/SessionRepository/list
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/services/session/test_list_sessions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/load_sessions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/integration_test/test_end_to_end_session_management
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn list_sessions(&self, options: SessionListOptions) -> Result<Vec<Session>>`

# Calls

- [new](../../../../../functions/src/db/repository/session/SessionRepository/new.md)
- [pool](../../../../../functions/src/services/ServiceContext/pool.md)
- [list](../../../../../functions/src/db/repository/session/SessionRepository/list.md)

# Called by

- [test_list_sessions](../../../../../functions/src/services/session/test_list_sessions.md)
- [load_sessions](../../../../../functions/src/tui/app/App/load_sessions.md)
- [test_end_to_end_session_management](../../../../../functions/tests/integration_test/test_end_to_end_session_management.md)