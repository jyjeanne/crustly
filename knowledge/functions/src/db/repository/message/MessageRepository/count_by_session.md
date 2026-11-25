---
type: Rust Method
title: count_by_session
resource: src/db/repository/message.rs#L169-L177
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/db/repository/message/test_message_list_by_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/message/MessageService/count_messages_in_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn count_by_session(&self, session_id: Uuid) -> Result<i64>`

# Calls

- [context](../../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [test_message_list_by_session](../../../../../../functions/src/db/repository/message/test_message_list_by_session.md)
- [count_messages_in_session](../../../../../../functions/src/services/message/MessageService/count_messages_in_session.md)