---
type: Rust Function
title: test_error_database_concurrent_access
resource: tests/error_scenarios_test.rs#L261-L287
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/services/ServiceContext/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/create_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/SessionService/get_session
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_error_database_concurrent_access() -> Result<()>`

# Calls

- [new](../../../functions/src/services/ServiceContext/new.md)
- [pool](../../../functions/src/db/Database/pool.md)
- [new](../../../functions/src/services/session/SessionService/new.md)
- [create_session](../../../functions/src/services/session/SessionService/create_session.md)
- [get_session](../../../functions/src/services/session/SessionService/get_session.md)