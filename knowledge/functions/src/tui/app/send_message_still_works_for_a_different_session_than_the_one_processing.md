---
type: Rust Function
title: send_message_still_works_for_a_different_session_than_the_one_processing
resource: src/tui/app.rs#L5994-L6013
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

`async fn send_message_still_works_for_a_different_session_than_the_one_processing()`

# Calls

- [create_new_session](../../../../functions/src/tui/app/App/create_new_session.md)
- [send_message](../../../../functions/src/tui/app/App/send_message.md)