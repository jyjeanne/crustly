---
type: Rust Method
title: archive
resource: src/db/repository/session.rs#L173-L186
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/db/repository/session/test_session_archive
  - functions/src/services/session/SessionService/archive_session
---

# Signature

`pub async fn archive(&self, id: Uuid) -> Result<()>`

# Called by

- [test_session_archive](../../../../../../functions/src/db/repository/session/test_session_archive.md)
- [archive_session](../../../../../../functions/src/services/session/SessionService/archive_session.md)