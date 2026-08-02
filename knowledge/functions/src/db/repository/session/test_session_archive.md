---
type: Rust Function
title: test_session_archive
resource: src/db/repository/session.rs#L302-L333
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/db/Database/run_migrations
  - functions/src/db/repository/session/SessionRepository/archive
  - functions/src/db/repository/session/SessionRepository/unarchive
---

# Signature

`async fn test_session_archive()`

# Calls

- [run_migrations](../../../../../functions/src/db/Database/run_migrations.md)
- [archive](../../../../../functions/src/db/repository/session/SessionRepository/archive.md)
- [unarchive](../../../../../functions/src/db/repository/session/SessionRepository/unarchive.md)