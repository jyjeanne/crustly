---
type: Rust Method
title: handle_event
resource: src/tui/app.rs#L633-L1005
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/tui/app/App/handle_key_event
  - functions/src/tui/app/App/event_belongs_to_current_session
  - functions/src/tui/app/App/append_streaming_chunk
  - functions/src/tui/app/App/complete_response
  - functions/src/tui/app/App/fail_current_plan_task
  - functions/src/tui/app/App/show_error
  - functions/src/tui/app/App/switch_mode
  - functions/src/tui/app/App/load_session
  - functions/src/tui/app/App/create_new_session
  - functions/src/tui/events/ToolApprovalRequest/is_timed_out
  - functions/src/tui/app/App/event_sender
  - functions/src/tui/app/App/handle_approval_requested
  - functions/src/tui/app/App/refresh_model_download_suggestions
  - functions/src/config/secrets/SecretString/len
  - functions/src/llm/agent/service/AgentService/set_provider
  - functions/src/services/session/SessionService/update_session
  called_by:
  - functions/src/tui/app/handle_ollama_models_listed_updates_installed_list
  - functions/src/tui/app/handle_ollama_pull_progress_updates_status_and_fraction
  - functions/src/tui/app/handle_ollama_pull_finished_success_posts_chat_message
  - functions/src/tui/app/handle_ollama_pull_finished_failure_posts_error_message
  - functions/src/tui/app/delete_key_on_installed_model_asks_for_confirmation
  - functions/src/tui/app/handle_ollama_delete_finished_success_removes_from_installed_and_posts_message
  - functions/src/tui/app/handle_ollama_delete_finished_failure_keeps_installed_and_posts_error
  - functions/src/tui/app/key_release_events_are_ignored
  - functions/src/tui/app/paste_preserves_backslashes_and_newlines
  - functions/src/tui/app/paste_inserts_at_cursor_not_always_appended_at_the_end
  - functions/src/tui/app/paste_with_embedded_newline_produces_multiple_lines
  - functions/src/tui/app/provider_switch_models_listed_clears_loading_state
  - functions/src/tui/app/llama_cpp_models_listed_clears_loading_state
  - functions/src/tui/app/llama_cpp_delete_finished_removes_model_from_list
  - functions/src/tui/app/llama_cpp_switch_finished_swaps_provider_in_place
  - functions/src/tui/app/llama_cpp_switch_finished_with_error_reports_failure_without_swapping
  - functions/src/tui/app/stale_session_response_chunk_is_dropped_after_switching_sessions
  - functions/src/tui/app/plan_task_error_marks_task_failed_and_stops_auto_execution
  - functions/src/tui/app/stale_session_response_complete_is_dropped_after_switching_sessions
  - functions/src/tui/runner/run_loop
---

# Signature

`pub async fn handle_event(&mut self, event: TuiEvent) -> Result<()>`

# Calls

- [handle_key_event](../../../../../functions/src/tui/app/App/handle_key_event.md)
- [event_belongs_to_current_session](../../../../../functions/src/tui/app/App/event_belongs_to_current_session.md)
- [append_streaming_chunk](../../../../../functions/src/tui/app/App/append_streaming_chunk.md)
- [complete_response](../../../../../functions/src/tui/app/App/complete_response.md)
- [fail_current_plan_task](../../../../../functions/src/tui/app/App/fail_current_plan_task.md)
- [show_error](../../../../../functions/src/tui/app/App/show_error.md)
- [switch_mode](../../../../../functions/src/tui/app/App/switch_mode.md)
- [load_session](../../../../../functions/src/tui/app/App/load_session.md)
- [create_new_session](../../../../../functions/src/tui/app/App/create_new_session.md)
- [is_timed_out](../../../../../functions/src/tui/events/ToolApprovalRequest/is_timed_out.md)
- [event_sender](../../../../../functions/src/tui/app/App/event_sender.md)
- [handle_approval_requested](../../../../../functions/src/tui/app/App/handle_approval_requested.md)
- [refresh_model_download_suggestions](../../../../../functions/src/tui/app/App/refresh_model_download_suggestions.md)
- [len](../../../../../functions/src/config/secrets/SecretString/len.md)
- [set_provider](../../../../../functions/src/llm/agent/service/AgentService/set_provider.md)
- [update_session](../../../../../functions/src/services/session/SessionService/update_session.md)

# Called by

- [handle_ollama_models_listed_updates_installed_list](../../../../../functions/src/tui/app/handle_ollama_models_listed_updates_installed_list.md)
- [handle_ollama_pull_progress_updates_status_and_fraction](../../../../../functions/src/tui/app/handle_ollama_pull_progress_updates_status_and_fraction.md)
- [handle_ollama_pull_finished_success_posts_chat_message](../../../../../functions/src/tui/app/handle_ollama_pull_finished_success_posts_chat_message.md)
- [handle_ollama_pull_finished_failure_posts_error_message](../../../../../functions/src/tui/app/handle_ollama_pull_finished_failure_posts_error_message.md)
- [delete_key_on_installed_model_asks_for_confirmation](../../../../../functions/src/tui/app/delete_key_on_installed_model_asks_for_confirmation.md)
- [handle_ollama_delete_finished_success_removes_from_installed_and_posts_message](../../../../../functions/src/tui/app/handle_ollama_delete_finished_success_removes_from_installed_and_posts_message.md)
- [handle_ollama_delete_finished_failure_keeps_installed_and_posts_error](../../../../../functions/src/tui/app/handle_ollama_delete_finished_failure_keeps_installed_and_posts_error.md)
- [key_release_events_are_ignored](../../../../../functions/src/tui/app/key_release_events_are_ignored.md)
- [paste_preserves_backslashes_and_newlines](../../../../../functions/src/tui/app/paste_preserves_backslashes_and_newlines.md)
- [paste_inserts_at_cursor_not_always_appended_at_the_end](../../../../../functions/src/tui/app/paste_inserts_at_cursor_not_always_appended_at_the_end.md)
- [paste_with_embedded_newline_produces_multiple_lines](../../../../../functions/src/tui/app/paste_with_embedded_newline_produces_multiple_lines.md)
- [provider_switch_models_listed_clears_loading_state](../../../../../functions/src/tui/app/provider_switch_models_listed_clears_loading_state.md)
- [llama_cpp_models_listed_clears_loading_state](../../../../../functions/src/tui/app/llama_cpp_models_listed_clears_loading_state.md)
- [llama_cpp_delete_finished_removes_model_from_list](../../../../../functions/src/tui/app/llama_cpp_delete_finished_removes_model_from_list.md)
- [llama_cpp_switch_finished_swaps_provider_in_place](../../../../../functions/src/tui/app/llama_cpp_switch_finished_swaps_provider_in_place.md)
- [llama_cpp_switch_finished_with_error_reports_failure_without_swapping](../../../../../functions/src/tui/app/llama_cpp_switch_finished_with_error_reports_failure_without_swapping.md)
- [stale_session_response_chunk_is_dropped_after_switching_sessions](../../../../../functions/src/tui/app/stale_session_response_chunk_is_dropped_after_switching_sessions.md)
- [plan_task_error_marks_task_failed_and_stops_auto_execution](../../../../../functions/src/tui/app/plan_task_error_marks_task_failed_and_stops_auto_execution.md)
- [stale_session_response_complete_is_dropped_after_switching_sessions](../../../../../functions/src/tui/app/stale_session_response_complete_is_dropped_after_switching_sessions.md)
- [run_loop](../../../../../functions/src/tui/runner/run_loop.md)