---
type: Rust Method
title: get_message
resource: src/services/message.rs#L64-L67
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
  - target: functions/src/db/repository/message/MessageRepository/find_by_id
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/services/message/MessageService/get_message_required
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/test_get_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/test_delete_message
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn get_message(&self, id: Uuid) -> Result<Option<Message>>`

# Calls

- [new](../../../../../functions/src/db/repository/message/MessageRepository/new.md)
- [pool](../../../../../functions/src/services/ServiceContext/pool.md)
- [find_by_id](../../../../../functions/src/db/repository/message/MessageRepository/find_by_id.md)
- [context](../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [get_message_required](../../../../../functions/src/services/message/MessageService/get_message_required.md)
- [test_get_message](../../../../../functions/src/services/message/test_get_message.md)
- [test_delete_message](../../../../../functions/src/services/message/test_delete_message.md)