---
type: Rust Function
title: test_pool_connect_in_memory
resource: src/db/mod.rs#L294-L297
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/SqlitePool/poolext/connect_in_memory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/SqlitePool/poolext/is_connected
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn test_pool_connect_in_memory()`

# Calls

- [connect_in_memory](../../../functions/src/db/SqlitePool/poolext/connect_in_memory.md)
- [is_connected](../../../functions/src/db/SqlitePool/poolext/is_connected.md)