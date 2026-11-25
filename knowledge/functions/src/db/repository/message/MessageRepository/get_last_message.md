---
type: Rust Method
title: get_last_message
resource: src/db/repository/message.rs#L180-L190
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn get_last_message(&self, session_id: Uuid) -> Result<Option<Message>>`

# Calls

- [context](../../../../../../functions/src/services/ServiceManager/context.md)