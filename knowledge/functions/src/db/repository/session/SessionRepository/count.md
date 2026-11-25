---
type: Rust Method
title: count
resource: src/db/repository/session.rs#L228-L241
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/services/session/SessionService/count_sessions
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/count_archived_sessions
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn count(&self, archived_only: bool) -> Result<i64>`

# Calls

- [context](../../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [count_sessions](../../../../../../functions/src/services/session/SessionService/count_sessions.md)
- [count_archived_sessions](../../../../../../functions/src/services/session/SessionService/count_archived_sessions.md)