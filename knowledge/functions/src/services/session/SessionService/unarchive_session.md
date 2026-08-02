---
type: Rust Method
title: unarchive_session
resource: src/services/session.rs#L135-L143
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/db/repository/session/SessionRepository/unarchive
  called_by:
  - functions/src/services/session/test_archive_unarchive_session
---

# Signature

`pub async fn unarchive_session(&self, id: Uuid) -> Result<()>`

# Calls

- [unarchive](../../../../../functions/src/db/repository/session/SessionRepository/unarchive.md)

# Called by

- [test_archive_unarchive_session](../../../../../functions/src/services/session/test_archive_unarchive_session.md)