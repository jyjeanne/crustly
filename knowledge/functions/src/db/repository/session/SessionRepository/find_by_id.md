---
type: Rust Method
title: find_by_id
resource: src/db/repository/session.rs#L35-L43
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/db/repository/session/test_session_crud
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/session/test_session_archive
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/get_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Session>>`

# Calls

- [context](../../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [test_session_crud](../../../../../../functions/src/db/repository/session/test_session_crud.md)
- [test_session_archive](../../../../../../functions/src/db/repository/session/test_session_archive.md)
- [get_session](../../../../../../functions/src/services/session/SessionService/get_session.md)