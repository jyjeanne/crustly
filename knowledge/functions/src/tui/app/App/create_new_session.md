---
type: Rust Method
title: create_new_session
resource: src/tui/app.rs#L1171-L1197
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/services/session/SessionService/update_session
  - functions/src/tui/app/App/sync_processing_state_for_current_session
  - functions/src/tui/app/App/load_sessions
  called_by:
  - functions/src/tui/app/App/initialize
  - functions/src/tui/app/App/handle_event
  - functions/src/tui/app/App/handle_key_event
  - functions/src/tui/app/stale_session_response_chunk_is_dropped_after_switching_sessions
  - functions/src/tui/app/plan_task_error_marks_task_failed_and_stops_auto_execution
  - functions/src/tui/app/stale_session_response_complete_is_dropped_after_switching_sessions
  - functions/src/tui/app/switching_sessions_clears_a_stuck_processing_state_from_the_previous_session
  - functions/src/tui/app/clear_session_is_refused_while_the_current_session_is_processing
  - functions/src/tui/app/clear_session_proceeds_when_only_another_session_is_processing
  - functions/src/tui/app/switching_back_to_a_session_with_a_still_in_flight_request_restores_processing_state
  - functions/src/tui/app/send_message_is_a_no_op_while_a_request_for_the_same_session_is_in_flight
  - functions/src/tui/app/send_message_still_works_for_a_different_session_than_the_one_processing
---

# Signature

`async fn create_new_session(&mut self) -> Result<()>`

# Calls

- [update_session](../../../../../functions/src/services/session/SessionService/update_session.md)
- [sync_processing_state_for_current_session](../../../../../functions/src/tui/app/App/sync_processing_state_for_current_session.md)
- [load_sessions](../../../../../functions/src/tui/app/App/load_sessions.md)

# Called by

- [initialize](../../../../../functions/src/tui/app/App/initialize.md)
- [handle_event](../../../../../functions/src/tui/app/App/handle_event.md)
- [handle_key_event](../../../../../functions/src/tui/app/App/handle_key_event.md)
- [stale_session_response_chunk_is_dropped_after_switching_sessions](../../../../../functions/src/tui/app/stale_session_response_chunk_is_dropped_after_switching_sessions.md)
- [plan_task_error_marks_task_failed_and_stops_auto_execution](../../../../../functions/src/tui/app/plan_task_error_marks_task_failed_and_stops_auto_execution.md)
- [stale_session_response_complete_is_dropped_after_switching_sessions](../../../../../functions/src/tui/app/stale_session_response_complete_is_dropped_after_switching_sessions.md)
- [switching_sessions_clears_a_stuck_processing_state_from_the_previous_session](../../../../../functions/src/tui/app/switching_sessions_clears_a_stuck_processing_state_from_the_previous_session.md)
- [clear_session_is_refused_while_the_current_session_is_processing](../../../../../functions/src/tui/app/clear_session_is_refused_while_the_current_session_is_processing.md)
- [clear_session_proceeds_when_only_another_session_is_processing](../../../../../functions/src/tui/app/clear_session_proceeds_when_only_another_session_is_processing.md)
- [switching_back_to_a_session_with_a_still_in_flight_request_restores_processing_state](../../../../../functions/src/tui/app/switching_back_to_a_session_with_a_still_in_flight_request_restores_processing_state.md)
- [send_message_is_a_no_op_while_a_request_for_the_same_session_is_in_flight](../../../../../functions/src/tui/app/send_message_is_a_no_op_while_a_request_for_the_same_session_is_in_flight.md)
- [send_message_still_works_for_a_different_session_than_the_one_processing](../../../../../functions/src/tui/app/send_message_still_works_for_a_different_session_than_the_one_processing.md)