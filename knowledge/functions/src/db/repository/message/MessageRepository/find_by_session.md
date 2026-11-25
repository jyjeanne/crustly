---
type: Rust Method
title: find_by_session
resource: src/db/repository/message.rs#L34-L44
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/db/repository/message/MessageRepository/list_by_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/message/MessageService/list_messages_for_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn find_by_session(&self, session_id: Uuid) -> Result<Vec<Message>>`

# Calls

- [context](../../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [list_by_session](../../../../../../functions/src/db/repository/message/MessageRepository/list_by_session.md)
- [list_messages_for_session](../../../../../../functions/src/services/message/MessageService/list_messages_for_session.md)