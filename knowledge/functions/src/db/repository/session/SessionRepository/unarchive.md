---
type: Rust Method
title: unarchive
resource: src/db/repository/session.rs#L189-L201
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/db/repository/session/test_session_archive
  - functions/src/services/session/SessionService/unarchive_session
---

# Signature

`pub async fn unarchive(&self, id: Uuid) -> Result<()>`

# Called by

- [test_session_archive](../../../../../../functions/src/db/repository/session/test_session_archive.md)
- [unarchive_session](../../../../../../functions/src/services/session/SessionService/unarchive_session.md)