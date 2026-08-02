---
type: Rust Method
title: delete_message
resource: src/services/message.rs#L145-L151
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/services/message/test_delete_message
---

# Signature

`pub async fn delete_message(&self, id: Uuid) -> Result<()>`

# Called by

- [test_delete_message](../../../../../functions/src/services/message/test_delete_message.md)