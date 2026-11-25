---
type: Rust Method
title: delete_message
resource: src/services/message.rs#L160-L166
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
  - target: functions/src/db/repository/message/MessageRepository/delete
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/services/message/test_delete_message
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn delete_message(&self, id: Uuid) -> Result<()>`

# Calls

- [new](../../../../../functions/src/db/repository/message/MessageRepository/new.md)
- [pool](../../../../../functions/src/services/ServiceContext/pool.md)
- [delete](../../../../../functions/src/db/repository/message/MessageRepository/delete.md)
- [context](../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [test_delete_message](../../../../../functions/src/services/message/test_delete_message.md)