---
type: Rust Function
title: bundled_sqlite_supports_fts5_secure_delete
resource: src/db/mod.rs#L369-L391
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/Database/connect_in_memory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/events/EventHandler/next
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn bundled_sqlite_supports_fts5_secure_delete()`

# Calls

- [connect_in_memory](../../../functions/src/db/Database/connect_in_memory.md)
- [pool](../../../functions/src/db/Database/pool.md)
- [next](../../../functions/src/tui/events/EventHandler/next.md)