---
type: Rust Function
title: stale_session_response_complete_is_dropped_after_switching_sessions
resource: src/tui/app.rs#L4516-L4557
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/tui/app/App/create_new_session
  - functions/src/config/secrets/SecretString/len
  - functions/src/tui/app/App/handle_event
---

# Signature

`async fn stale_session_response_complete_is_dropped_after_switching_sessions()`

# Calls

- [create_new_session](../../../../functions/src/tui/app/App/create_new_session.md)
- [len](../../../../functions/src/config/secrets/SecretString/len.md)
- [handle_event](../../../../functions/src/tui/app/App/handle_event.md)