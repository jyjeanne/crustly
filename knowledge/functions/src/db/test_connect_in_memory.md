---
type: Rust Function
title: test_connect_in_memory
resource: src/db/mod.rs#L288-L291
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/Database/connect_in_memory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/is_connected
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn test_connect_in_memory()`

# Calls

- [connect_in_memory](../../../functions/src/db/Database/connect_in_memory.md)
- [is_connected](../../../functions/src/db/Database/is_connected.md)