---
type: Rust Method
title: create_new_session
resource: src/tui/app.rs#L2263-L2290
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/app/App/end_session_in_background
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/SessionService/create_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/provider_model
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/provider_name
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/update_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/sync_processing_state_for_current_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/load_sessions
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/initialize
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_key_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/slash_review_resolves_to_the_builtin_skill
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/slash_review_with_arguments_appends_them_to_the_skill_prompt
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/slash_spec_resolves_to_the_builtin_skill_with_arguments
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/stale_session_response_chunk_is_dropped_after_switching_sessions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/plan_task_error_marks_task_failed_and_stops_auto_execution
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/stale_session_response_complete_is_dropped_after_switching_sessions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/switching_sessions_clears_a_stuck_processing_state_from_the_previous_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/clear_session_is_refused_while_the_current_session_is_processing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/clear_session_proceeds_when_only_another_session_is_processing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/switching_back_to_a_session_with_a_still_in_flight_request_restores_processing_state
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/send_message_is_a_no_op_while_a_request_for_the_same_session_is_in_flight
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/send_message_still_works_for_a_different_session_than_the_one_processing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/app_with_extraction
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/starting_a_new_session_ends_the_previous_one
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn create_new_session(&mut self) -> Result<()>`

# Calls

- [end_session_in_background](../../../../../functions/src/tui/app/App/end_session_in_background.md)
- [create_session](../../../../../functions/src/services/session/SessionService/create_session.md)
- [provider_model](../../../../../functions/src/llm/agent/service/AgentService/provider_model.md)
- [provider_name](../../../../../functions/src/llm/agent/service/AgentService/provider_name.md)
- [update_session](../../../../../functions/src/services/session/SessionService/update_session.md)
- [sync_processing_state_for_current_session](../../../../../functions/src/tui/app/App/sync_processing_state_for_current_session.md)
- [load_sessions](../../../../../functions/src/tui/app/App/load_sessions.md)

# Called by

- [initialize](../../../../../functions/src/tui/app/App/initialize.md)
- [handle_event](../../../../../functions/src/tui/app/App/handle_event.md)
- [handle_key_event](../../../../../functions/src/tui/app/App/handle_key_event.md)
- [slash_review_resolves_to_the_builtin_skill](../../../../../functions/src/tui/app/slash_review_resolves_to_the_builtin_skill.md)
- [slash_review_with_arguments_appends_them_to_the_skill_prompt](../../../../../functions/src/tui/app/slash_review_with_arguments_appends_them_to_the_skill_prompt.md)
- [slash_spec_resolves_to_the_builtin_skill_with_arguments](../../../../../functions/src/tui/app/slash_spec_resolves_to_the_builtin_skill_with_arguments.md)
- [stale_session_response_chunk_is_dropped_after_switching_sessions](../../../../../functions/src/tui/app/stale_session_response_chunk_is_dropped_after_switching_sessions.md)
- [plan_task_error_marks_task_failed_and_stops_auto_execution](../../../../../functions/src/tui/app/plan_task_error_marks_task_failed_and_stops_auto_execution.md)
- [stale_session_response_complete_is_dropped_after_switching_sessions](../../../../../functions/src/tui/app/stale_session_response_complete_is_dropped_after_switching_sessions.md)
- [switching_sessions_clears_a_stuck_processing_state_from_the_previous_session](../../../../../functions/src/tui/app/switching_sessions_clears_a_stuck_processing_state_from_the_previous_session.md)
- [clear_session_is_refused_while_the_current_session_is_processing](../../../../../functions/src/tui/app/clear_session_is_refused_while_the_current_session_is_processing.md)
- [clear_session_proceeds_when_only_another_session_is_processing](../../../../../functions/src/tui/app/clear_session_proceeds_when_only_another_session_is_processing.md)
- [switching_back_to_a_session_with_a_still_in_flight_request_restores_processing_state](../../../../../functions/src/tui/app/switching_back_to_a_session_with_a_still_in_flight_request_restores_processing_state.md)
- [send_message_is_a_no_op_while_a_request_for_the_same_session_is_in_flight](../../../../../functions/src/tui/app/send_message_is_a_no_op_while_a_request_for_the_same_session_is_in_flight.md)
- [send_message_still_works_for_a_different_session_than_the_one_processing](../../../../../functions/src/tui/app/send_message_still_works_for_a_different_session_than_the_one_processing.md)
- [app_with_extraction](../../../../../functions/src/tui/app/app_with_extraction.md)
- [starting_a_new_session_ends_the_previous_one](../../../../../functions/src/tui/app/starting_a_new_session_ends_the_previous_one.md)