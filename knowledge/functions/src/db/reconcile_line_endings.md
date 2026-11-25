---
type: Rust Function
title: reconcile_line_endings
resource: src/db/mod.rs#L242-L281
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/other_line_endings
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/db/Database/run_migrations
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn reconcile_line_endings( conn: &mut sqlx::SqliteConnection, migrator: &sqlx::migrate::Migrator, ) -> anyhow::Result<()>`

# Calls

- [other_line_endings](../../../functions/src/db/other_line_endings.md)

# Called by

- [run_migrations](../../../functions/src/db/Database/run_migrations.md)