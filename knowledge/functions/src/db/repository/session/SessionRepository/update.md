---
type: Rust Method
title: update
resource: src/db/repository/session.rs#L72-L95
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/db/repository/session/test_session_crud
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/update_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/update_session_title
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/update_session_usage
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn update(&self, session: &Session) -> Result<()>`

# Calls

- [context](../../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [test_session_crud](../../../../../../functions/src/db/repository/session/test_session_crud.md)
- [update_session](../../../../../../functions/src/services/session/SessionService/update_session.md)
- [update_session_title](../../../../../../functions/src/services/session/SessionService/update_session_title.md)
- [update_session_usage](../../../../../../functions/src/services/session/SessionService/update_session_usage.md)