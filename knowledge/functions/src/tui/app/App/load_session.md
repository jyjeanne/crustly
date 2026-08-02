---
type: Rust Method
title: load_session
resource: src/tui/app.rs#L1465-L1494
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/services/session/SessionService/get_session
  - functions/src/services/message/MessageService/list_messages_for_session
  - functions/src/services/session/SessionService/update_session
  - functions/src/tui/app/App/sync_processing_state_for_current_session
  called_by:
  - functions/src/tui/app/App/initialize
  - functions/src/tui/app/App/handle_event
  - functions/src/tui/app/App/handle_sessions_key
  - functions/src/tui/app/switching_back_to_a_session_with_a_still_in_flight_request_restores_processing_state
---

# Signature

`async fn load_session(&mut self, session_id: Uuid) -> Result<()>`

# Calls

- [get_session](../../../../../functions/src/services/session/SessionService/get_session.md)
- [list_messages_for_session](../../../../../functions/src/services/message/MessageService/list_messages_for_session.md)
- [update_session](../../../../../functions/src/services/session/SessionService/update_session.md)
- [sync_processing_state_for_current_session](../../../../../functions/src/tui/app/App/sync_processing_state_for_current_session.md)

# Called by

- [initialize](../../../../../functions/src/tui/app/App/initialize.md)
- [handle_event](../../../../../functions/src/tui/app/App/handle_event.md)
- [handle_sessions_key](../../../../../functions/src/tui/app/App/handle_sessions_key.md)
- [switching_back_to_a_session_with_a_still_in_flight_request_restores_processing_state](../../../../../functions/src/tui/app/switching_back_to_a_session_with_a_still_in_flight_request_restores_processing_state.md)