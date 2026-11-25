---
type: Rust Function
title: test_service_context_creation
resource: src/services/mod.rs#L103-L107
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/services/create_test_pool
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/ServiceContext/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceContext/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn test_service_context_creation()`

# Calls

- [create_test_pool](../../../functions/src/services/create_test_pool.md)
- [new](../../../functions/src/services/ServiceContext/new.md)
- [pool](../../../functions/src/services/ServiceContext/pool.md)