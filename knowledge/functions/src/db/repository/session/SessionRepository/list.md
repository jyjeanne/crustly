---
type: Rust Method
title: list
resource: src/db/repository/session.rs#L110-L146
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/services/session/SessionService/list_sessions
  - functions/src/services/session/SessionService/get_most_recent_session
---

# Signature

`pub async fn list(&self, options: SessionListOptions) -> Result<Vec<Session>>`

# Called by

- [list_sessions](../../../../../../functions/src/services/session/SessionService/list_sessions.md)
- [get_most_recent_session](../../../../../../functions/src/services/session/SessionService/get_most_recent_session.md)