---
type: Rust Method
title: get_most_recent_session
resource: src/services/session.rs#L153-L163
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/repository/session/SessionRepository/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceContext/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/session/SessionRepository/list
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/events/EventHandler/next
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/services/session/test_get_most_recent_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/initialize
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn get_most_recent_session(&self) -> Result<Option<Session>>`

# Calls

- [new](../../../../../functions/src/db/repository/session/SessionRepository/new.md)
- [pool](../../../../../functions/src/services/ServiceContext/pool.md)
- [list](../../../../../functions/src/db/repository/session/SessionRepository/list.md)
- [next](../../../../../functions/src/tui/events/EventHandler/next.md)

# Called by

- [test_get_most_recent_session](../../../../../functions/src/services/session/test_get_most_recent_session.md)
- [initialize](../../../../../functions/src/tui/app/App/initialize.md)