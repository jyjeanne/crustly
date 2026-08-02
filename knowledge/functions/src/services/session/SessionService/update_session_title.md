---
type: Rust Method
title: update_session_title
resource: src/services/session.rs#L88-L100
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/services/session/SessionService/get_session_required
  called_by:
  - functions/src/services/session/test_update_session_title
---

# Signature

`pub async fn update_session_title(&self, id: Uuid, title: Option<String>) -> Result<()>`

# Calls

- [get_session_required](../../../../../functions/src/services/session/SessionService/get_session_required.md)

# Called by

- [test_update_session_title](../../../../../functions/src/services/session/test_update_session_title.md)