---
type: Rust Method
title: update_session_title
resource: src/services/session.rs#L86-L98
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/services/session/SessionService/get_session_required
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/session/SessionRepository/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceContext/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/session/SessionRepository/update
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/services/session/test_update_session_title
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn update_session_title(&self, id: Uuid, title: Option<String>) -> Result<()>`

# Calls

- [get_session_required](../../../../../functions/src/services/session/SessionService/get_session_required.md)
- [new](../../../../../functions/src/db/repository/session/SessionRepository/new.md)
- [pool](../../../../../functions/src/services/ServiceContext/pool.md)
- [update](../../../../../functions/src/db/repository/session/SessionRepository/update.md)
- [context](../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [test_update_session_title](../../../../../functions/src/services/session/test_update_session_title.md)