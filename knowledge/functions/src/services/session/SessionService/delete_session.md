---
type: Rust Method
title: delete_session
resource: src/services/session.rs#L144-L150
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/repository/session/SessionRepository/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceContext/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/session/SessionRepository/delete
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/services/session/test_delete_session
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn delete_session(&self, id: Uuid) -> Result<()>`

# Calls

- [new](../../../../../functions/src/db/repository/session/SessionRepository/new.md)
- [pool](../../../../../functions/src/services/ServiceContext/pool.md)
- [delete](../../../../../functions/src/db/repository/session/SessionRepository/delete.md)
- [context](../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [test_delete_session](../../../../../functions/src/services/session/test_delete_session.md)