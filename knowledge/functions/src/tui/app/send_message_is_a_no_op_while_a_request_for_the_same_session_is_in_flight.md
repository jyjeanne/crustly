---
type: Rust Function
title: send_message_is_a_no_op_while_a_request_for_the_same_session_is_in_flight
resource: src/tui/app.rs#L5970-L5988
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/app/App/create_new_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/send_message
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn send_message_is_a_no_op_while_a_request_for_the_same_session_is_in_flight()`

# Calls

- [create_new_session](../../../../functions/src/tui/app/App/create_new_session.md)
- [send_message](../../../../functions/src/tui/app/App/send_message.md)