---
type: Rust Method
title: update_message
resource: src/services/message.rs#L85-L93
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/repository/message/MessageRepository/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceContext/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/message/MessageRepository/update
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn update_message(&self, message: &Message) -> Result<()>`

# Calls

- [new](../../../../../functions/src/db/repository/message/MessageRepository/new.md)
- [pool](../../../../../functions/src/services/ServiceContext/pool.md)
- [update](../../../../../functions/src/db/repository/message/MessageRepository/update.md)
- [context](../../../../../functions/src/services/ServiceManager/context.md)