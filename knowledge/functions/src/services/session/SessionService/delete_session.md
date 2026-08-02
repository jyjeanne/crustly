---
type: Rust Method
title: delete_session
resource: src/services/session.rs#L146-L152
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/services/session/test_delete_session
---

# Signature

`pub async fn delete_session(&self, id: Uuid) -> Result<()>`

# Called by

- [test_delete_session](../../../../../functions/src/services/session/test_delete_session.md)