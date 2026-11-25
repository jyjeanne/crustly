---
type: Rust Function
title: cmd_db
resource: src/cli/mod.rs#L1718-L1819
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/Database/connect
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/run_migrations
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/Database/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/cli/run
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn cmd_db(config: &crate::config::Config, operation: DbCommands) -> Result<()>`

# Calls

- [connect](../../../functions/src/db/Database/connect.md)
- [run_migrations](../../../functions/src/db/Database/run_migrations.md)
- [pool](../../../functions/src/db/Database/pool.md)

# Called by

- [run](../../../functions/src/cli/run.md)