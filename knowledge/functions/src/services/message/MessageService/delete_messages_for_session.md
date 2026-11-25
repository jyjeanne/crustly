---
type: Rust Method
title: delete_messages_for_session
resource: src/services/message.rs#L169-L177
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/repository/message/MessageRepository/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceContext/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/message/MessageRepository/delete_by_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/services/message/test_delete_messages_for_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/clear_session
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn delete_messages_for_session(&self, session_id: Uuid) -> Result<()>`

# Calls

- [new](../../../../../functions/src/db/repository/message/MessageRepository/new.md)
- [pool](../../../../../functions/src/services/ServiceContext/pool.md)
- [delete_by_session](../../../../../functions/src/db/repository/message/MessageRepository/delete_by_session.md)
- [context](../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [test_delete_messages_for_session](../../../../../functions/src/services/message/test_delete_messages_for_session.md)
- [clear_session](../../../../../functions/src/tui/app/App/clear_session.md)