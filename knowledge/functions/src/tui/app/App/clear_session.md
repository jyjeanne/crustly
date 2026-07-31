---
type: Rust Method
title: clear_session
resource: src/tui/app.rs#L1248-L1279
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/services/message/MessageService/delete_messages_for_session
  called_by:
  - functions/src/tui/app/App/handle_key_event
  - functions/src/tui/app/clear_session_is_refused_while_the_current_session_is_processing
  - functions/src/tui/app/clear_session_proceeds_when_only_another_session_is_processing
---

# Signature

`async fn clear_session(&mut self) -> Result<()>`

# Calls

- [delete_messages_for_session](../../../../../functions/src/services/message/MessageService/delete_messages_for_session.md)

# Called by

- [handle_key_event](../../../../../functions/src/tui/app/App/handle_key_event.md)
- [clear_session_is_refused_while_the_current_session_is_processing](../../../../../functions/src/tui/app/clear_session_is_refused_while_the_current_session_is_processing.md)
- [clear_session_proceeds_when_only_another_session_is_processing](../../../../../functions/src/tui/app/clear_session_proceeds_when_only_another_session_is_processing.md)