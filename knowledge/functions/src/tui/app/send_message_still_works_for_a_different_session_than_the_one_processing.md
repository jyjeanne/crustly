---
type: Rust Function
title: send_message_still_works_for_a_different_session_than_the_one_processing
resource: src/tui/app.rs#L4079-L4098
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/tui/app/App/create_new_session
  - functions/src/config/secrets/SecretString/len
---

# Signature

`async fn send_message_still_works_for_a_different_session_than_the_one_processing()`

# Calls

- [create_new_session](../../../../functions/src/tui/app/App/create_new_session.md)
- [len](../../../../functions/src/config/secrets/SecretString/len.md)