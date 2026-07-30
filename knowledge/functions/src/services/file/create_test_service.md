---
type: Rust Function
title: create_test_service
resource: src/services/file.rs#L176-L188
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/db/Database/run_migrations
---

# Signature

`async fn create_test_service() -> (FileService, SessionService)`

# Calls

- [run_migrations](../../../../functions/src/db/Database/run_migrations.md)