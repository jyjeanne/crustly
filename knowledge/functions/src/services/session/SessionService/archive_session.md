---
type: Rust Method
title: archive_session
resource: src/services/session.rs#L122-L130
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
  - target: functions/src/db/repository/session/SessionRepository/archive
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/services/session/test_archive_unarchive_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/test_count_sessions
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn archive_session(&self, id: Uuid) -> Result<()>`

# Calls

- [new](../../../../../functions/src/db/repository/session/SessionRepository/new.md)
- [pool](../../../../../functions/src/services/ServiceContext/pool.md)
- [archive](../../../../../functions/src/db/repository/session/SessionRepository/archive.md)
- [context](../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [test_archive_unarchive_session](../../../../../functions/src/services/session/test_archive_unarchive_session.md)
- [test_count_sessions](../../../../../functions/src/services/session/test_count_sessions.md)