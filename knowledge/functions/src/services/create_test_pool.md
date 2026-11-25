---
type: Rust Function
title: create_test_pool
resource: src/services/mod.rs#L94-L100
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
  - target: functions/src/db/Database/run_migrations
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/Database/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/services/test_service_context_creation
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/test_service_manager_creation
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn create_test_pool() -> Pool`

# Calls

- [connect_in_memory](../../../functions/src/db/Database/connect_in_memory.md)
- [run_migrations](../../../functions/src/db/Database/run_migrations.md)
- [pool](../../../functions/src/db/Database/pool.md)

# Called by

- [test_service_context_creation](../../../functions/src/services/test_service_context_creation.md)
- [test_service_manager_creation](../../../functions/src/services/test_service_manager_creation.md)