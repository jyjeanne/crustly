---
type: Rust Method
title: unarchive
resource: src/db/repository/session.rs#L189-L201
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/db/repository/session/test_session_archive
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/SessionService/unarchive_session
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn unarchive(&self, id: Uuid) -> Result<()>`

# Calls

- [context](../../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [test_session_archive](../../../../../../functions/src/db/repository/session/test_session_archive.md)
- [unarchive_session](../../../../../../functions/src/services/session/SessionService/unarchive_session.md)