---
type: Rust Method
title: connect_in_memory
resource: src/db/mod.rs#L217-L220
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/Database/connect_in_memory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/db/test_pool_connect_in_memory
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn connect_in_memory() -> Result<Self>`

# Calls

- [connect_in_memory](../../../../../functions/src/db/Database/connect_in_memory.md)

# Called by

- [test_pool_connect_in_memory](../../../../../functions/src/db/test_pool_connect_in_memory.md)