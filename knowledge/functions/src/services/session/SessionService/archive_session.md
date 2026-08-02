---
type: Rust Method
title: archive_session
resource: src/services/session.rs#L124-L132
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/db/repository/session/SessionRepository/archive
  called_by:
  - functions/src/services/session/test_archive_unarchive_session
  - functions/src/services/session/test_count_sessions
---

# Signature

`pub async fn archive_session(&self, id: Uuid) -> Result<()>`

# Calls

- [archive](../../../../../functions/src/db/repository/session/SessionRepository/archive.md)

# Called by

- [test_archive_unarchive_session](../../../../../functions/src/services/session/test_archive_unarchive_session.md)
- [test_count_sessions](../../../../../functions/src/services/session/test_count_sessions.md)