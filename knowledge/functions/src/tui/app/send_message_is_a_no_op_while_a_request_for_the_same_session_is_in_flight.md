---
type: Rust Function
title: send_message_is_a_no_op_while_a_request_for_the_same_session_is_in_flight
resource: src/tui/app.rs#L4711-L4729
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/tui/app/App/create_new_session
  - functions/src/config/secrets/SecretString/len
---

# Signature

`async fn send_message_is_a_no_op_while_a_request_for_the_same_session_is_in_flight()`

# Calls

- [create_new_session](../../../../functions/src/tui/app/App/create_new_session.md)
- [len](../../../../functions/src/config/secrets/SecretString/len.md)