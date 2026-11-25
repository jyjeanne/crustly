---
type: Rust Method
title: delete
resource: src/db/repository/session.rs#L98-L107
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/db/repository/session/test_session_crud
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/delete_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn delete(&self, id: Uuid) -> Result<()>`

# Calls

- [context](../../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [test_session_crud](../../../../../../functions/src/db/repository/session/test_session_crud.md)
- [delete_session](../../../../../../functions/src/services/session/SessionService/delete_session.md)