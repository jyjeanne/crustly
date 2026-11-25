---
type: Rust Method
title: delete_by_session
resource: src/db/repository/message.rs#L193-L202
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/services/message/MessageService/delete_messages_for_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn delete_by_session(&self, session_id: Uuid) -> Result<()>`

# Calls

- [context](../../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [delete_messages_for_session](../../../../../../functions/src/services/message/MessageService/delete_messages_for_session.md)