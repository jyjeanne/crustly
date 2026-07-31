---
type: Rust Module
title: app
resource: src/app/mod.rs#L1-L106
generated:
  by: okf-rs/0.2.0
relationships:
  imports:
  - external/anyhow-result
  - external/notify-event-eventkind-recursivemode-watcher
  - external/sqlx-sqlitepool
  - external/std-path-path-pathbuf
  - external/std-sync-arc
  member_of:
  - packages/crustly
---

# Contains

- [App](../../classes/src/app/App.md)
- [new](../../functions/src/app/App/new.md)
- [run](../../functions/src/app/App/run.md)
- [default](../../functions/src/app/App/default/default.md)
- [start_file_watcher](../../functions/src/app/start_file_watcher.md)
- [is_rust_file_in_root](../../functions/src/app/is_rust_file_in_root.md)

# Imports

- `anyhow::Result`
- `notify::{Event, EventKind, RecursiveMode, Watcher}`
- `sqlx::SqlitePool`
- `std::path::{Path, PathBuf}`
- `std::sync::Arc`

# Member of

- [crustly](../../packages/crustly.md)