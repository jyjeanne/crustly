---
type: Rust Function
title: cmd_db
resource: src/cli/mod.rs#L490-L591
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/db/Database/run_migrations
  called_by:
  - functions/src/cli/run
---

# Signature

`async fn cmd_db(config: &crate::config::Config, operation: DbCommands) -> Result<()>`

# Calls

- [run_migrations](../../../functions/src/db/Database/run_migrations.md)

# Called by

- [run](../../../functions/src/cli/run.md)