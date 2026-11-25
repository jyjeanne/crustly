---
type: Rust Method
title: handle_event
resource: src/tui/app.rs#L1001-L1386
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/app/App/handle_key_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/paste_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/send_message
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/App/event_belongs_to_current_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/append_streaming_chunk
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/complete_response
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/announce_memory_saves
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/fail_current_plan_task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/show_error
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/switch_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/load_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/create_new_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/notice
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/ToolApprovalRequest/is_timed_out
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/event_sender
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_approval_requested
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/refresh_model_download_suggestions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/SkillSource/display
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/set_provider
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/refresh_context_usage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/SessionService/update_session
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/handle_ollama_models_listed_updates_installed_list
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/handle_ollama_pull_progress_updates_status_and_fraction
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/handle_ollama_pull_finished_success_posts_chat_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/handle_ollama_pull_finished_failure_posts_error_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/delete_key_on_installed_model_asks_for_confirmation
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/handle_ollama_delete_finished_success_removes_from_installed_and_posts_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/handle_ollama_delete_finished_failure_keeps_installed_and_posts_error
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/key_release_events_are_ignored
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/paste_preserves_backslashes_and_newlines
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/paste_inserts_at_cursor_not_always_appended_at_the_end
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/paste_with_embedded_newline_produces_multiple_lines
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/ctrl_c_clears_the_input_first_and_quits_on_a_second_press
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/ctrl_c_quits_only_if_the_second_press_comes_soon
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/a_multi_line_paste_is_inserted_whole_without_sending
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/a_paste_in_the_download_dialog_fills_its_input_with_one_line
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/ctrl_z_undoes_a_paste_and_ctrl_shift_z_redoes_it
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/a_paste_in_a_view_without_text_input_does_nothing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/help_scrolls_within_its_height
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/provider_switch_models_listed_clears_loading_state
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/llama_cpp_hardware_detected_clears_loading_state_and_stores_the_result
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/llama_cpp_models_listed_clears_loading_state
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/llama_cpp_delete_finished_removes_model_from_list
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/llama_cpp_switch_finished_swaps_provider_in_place
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/llama_cpp_switch_finished_with_error_reports_failure_without_swapping
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
  - target: functions/src/tui/runner/run_loop
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn handle_event(&mut self, event: TuiEvent) -> Result<()>`

# Calls

- [handle_key_event](../../../../../functions/src/tui/app/App/handle_key_event.md)
- [paste_text](../../../../../functions/src/tui/app/App/paste_text.md)
- [send_message](../../../../../functions/src/tui/app/App/send_message.md)
- [event_belongs_to_current_session](../../../../../functions/src/tui/app/App/event_belongs_to_current_session.md)
- [append_streaming_chunk](../../../../../functions/src/tui/app/App/append_streaming_chunk.md)
- [complete_response](../../../../../functions/src/tui/app/App/complete_response.md)
- [announce_memory_saves](../../../../../functions/src/tui/app/App/announce_memory_saves.md)
- [fail_current_plan_task](../../../../../functions/src/tui/app/App/fail_current_plan_task.md)
- [show_error](../../../../../functions/src/tui/app/App/show_error.md)
- [switch_mode](../../../../../functions/src/tui/app/App/switch_mode.md)
- [load_session](../../../../../functions/src/tui/app/App/load_session.md)
- [create_new_session](../../../../../functions/src/tui/app/App/create_new_session.md)
- [notice](../../../../../functions/src/tui/app/App/notice.md)
- [is_timed_out](../../../../../functions/src/tui/events/ToolApprovalRequest/is_timed_out.md)
- [event_sender](../../../../../functions/src/tui/app/App/event_sender.md)
- [handle_approval_requested](../../../../../functions/src/tui/app/App/handle_approval_requested.md)
- [refresh_model_download_suggestions](../../../../../functions/src/tui/app/App/refresh_model_download_suggestions.md)
- [display](../../../../../functions/src/llm/tools/skill/SkillSource/display.md)
- [set_provider](../../../../../functions/src/llm/agent/service/AgentService/set_provider.md)
- [refresh_context_usage](../../../../../functions/src/tui/app/App/refresh_context_usage.md)
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
- [ctrl_c_clears_the_input_first_and_quits_on_a_second_press](../../../../../functions/src/tui/app/ctrl_c_clears_the_input_first_and_quits_on_a_second_press.md)
- [ctrl_c_quits_only_if_the_second_press_comes_soon](../../../../../functions/src/tui/app/ctrl_c_quits_only_if_the_second_press_comes_soon.md)
- [a_multi_line_paste_is_inserted_whole_without_sending](../../../../../functions/src/tui/app/a_multi_line_paste_is_inserted_whole_without_sending.md)
- [a_paste_in_the_download_dialog_fills_its_input_with_one_line](../../../../../functions/src/tui/app/a_paste_in_the_download_dialog_fills_its_input_with_one_line.md)
- [ctrl_z_undoes_a_paste_and_ctrl_shift_z_redoes_it](../../../../../functions/src/tui/app/ctrl_z_undoes_a_paste_and_ctrl_shift_z_redoes_it.md)
- [a_paste_in_a_view_without_text_input_does_nothing](../../../../../functions/src/tui/app/a_paste_in_a_view_without_text_input_does_nothing.md)
- [help_scrolls_within_its_height](../../../../../functions/src/tui/app/help_scrolls_within_its_height.md)
- [provider_switch_models_listed_clears_loading_state](../../../../../functions/src/tui/app/provider_switch_models_listed_clears_loading_state.md)
- [llama_cpp_hardware_detected_clears_loading_state_and_stores_the_result](../../../../../functions/src/tui/app/llama_cpp_hardware_detected_clears_loading_state_and_stores_the_result.md)
- [llama_cpp_models_listed_clears_loading_state](../../../../../functions/src/tui/app/llama_cpp_models_listed_clears_loading_state.md)
- [llama_cpp_delete_finished_removes_model_from_list](../../../../../functions/src/tui/app/llama_cpp_delete_finished_removes_model_from_list.md)
- [llama_cpp_switch_finished_swaps_provider_in_place](../../../../../functions/src/tui/app/llama_cpp_switch_finished_swaps_provider_in_place.md)
- [llama_cpp_switch_finished_with_error_reports_failure_without_swapping](../../../../../functions/src/tui/app/llama_cpp_switch_finished_with_error_reports_failure_without_swapping.md)
- [stale_session_response_chunk_is_dropped_after_switching_sessions](../../../../../functions/src/tui/app/stale_session_response_chunk_is_dropped_after_switching_sessions.md)
- [plan_task_error_marks_task_failed_and_stops_auto_execution](../../../../../functions/src/tui/app/plan_task_error_marks_task_failed_and_stops_auto_execution.md)
- [stale_session_response_complete_is_dropped_after_switching_sessions](../../../../../functions/src/tui/app/stale_session_response_complete_is_dropped_after_switching_sessions.md)
- [run_loop](../../../../../functions/src/tui/runner/run_loop.md)