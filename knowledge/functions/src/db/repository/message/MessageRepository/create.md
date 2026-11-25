---
type: Rust Method
title: create
resource: src/db/repository/message.rs#L59-L124
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
  - target: functions/src/db/repository/message/test_message_list_by_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/message/MessageService/create_message
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn create(&self, message: &mut Message) -> Result<()>`

# Calls

- [context](../../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [test_message_crud](../../../../../../functions/src/db/repository/message/test_message_crud.md)
- [test_message_list_by_session](../../../../../../functions/src/db/repository/message/test_message_list_by_session.md)
- [create_message](../../../../../../functions/src/services/message/MessageService/create_message.md)