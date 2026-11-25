---
type: Rust Method
title: list
resource: src/db/repository/session.rs#L110-L146
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/services/session/SessionService/list_sessions
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/get_most_recent_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn list(&self, options: SessionListOptions) -> Result<Vec<Session>>`

# Calls

- [context](../../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [list_sessions](../../../../../../functions/src/services/session/SessionService/list_sessions.md)
- [get_most_recent_session](../../../../../../functions/src/services/session/SessionService/get_most_recent_session.md)