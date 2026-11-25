---
type: Rust Method
title: count_sessions
resource: src/services/session.rs#L166-L169
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/repository/session/SessionRepository/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceContext/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/session/SessionRepository/count
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/services/session/test_count_sessions
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn count_sessions(&self) -> Result<i64>`

# Calls

- [new](../../../../../functions/src/db/repository/session/SessionRepository/new.md)
- [pool](../../../../../functions/src/services/ServiceContext/pool.md)
- [count](../../../../../functions/src/db/repository/session/SessionRepository/count.md)

# Called by

- [test_count_sessions](../../../../../functions/src/services/session/test_count_sessions.md)