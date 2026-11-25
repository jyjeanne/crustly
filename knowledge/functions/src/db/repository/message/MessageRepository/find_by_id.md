---
type: Rust Method
title: find_by_id
resource: src/db/repository/message.rs#L23-L31
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/db/repository/message/test_message_crud
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/message/MessageService/get_message
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Message>>`

# Calls

- [context](../../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [test_message_crud](../../../../../../functions/src/db/repository/message/test_message_crud.md)
- [get_message](../../../../../../functions/src/services/message/MessageService/get_message.md)