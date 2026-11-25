---
type: Rust Method
title: delete
resource: src/db/repository/message.rs#L152-L161
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/db/repository/message/test_message_crud
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/message/MessageService/delete_message
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn delete(&self, id: Uuid) -> Result<()>`

# Calls

- [context](../../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [test_message_crud](../../../../../../functions/src/db/repository/message/test_message_crud.md)
- [delete_message](../../../../../../functions/src/services/message/MessageService/delete_message.md)