---
type: Rust Method
title: get_most_recent_session
resource: src/services/session.rs#L155-L165
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/db/repository/session/SessionRepository/list
  - functions/src/tui/events/EventHandler/next
  called_by:
  - functions/src/services/session/test_get_most_recent_session
  - functions/src/tui/app/App/initialize
---

# Signature

`pub async fn get_most_recent_session(&self) -> Result<Option<Session>>`

# Calls

- [list](../../../../../functions/src/db/repository/session/SessionRepository/list.md)
- [next](../../../../../functions/src/tui/events/EventHandler/next.md)

# Called by

- [test_get_most_recent_session](../../../../../functions/src/services/session/test_get_most_recent_session.md)
- [initialize](../../../../../functions/src/tui/app/App/initialize.md)