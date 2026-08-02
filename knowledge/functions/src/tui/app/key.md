---
type: Rust Function
title: key
resource: src/tui/app.rs#L3003-L3005
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/logging/LoggerGuard/empty
  called_by:
  - functions/src/tui/app/model_download_typing_filters_suggestions
  - functions/src/tui/app/model_download_backspace_removes_last_char
  - functions/src/tui/app/model_download_tab_adopts_highlighted_suggestion
  - functions/src/tui/app/model_download_esc_closes_dialog_without_running_pull
  - functions/src/tui/app/model_download_enter_starts_pull_then_esc_aborts_it
  - functions/src/tui/app/delete_key_ignored_for_uninstalled_suggestion
  - functions/src/tui/app/delete_key_on_installed_model_asks_for_confirmation
  - functions/src/tui/app/confirm_delete_n_cancels_back_to_list
  - functions/src/tui/app/confirm_delete_esc_cancels_back_to_list_without_closing_dialog
  - functions/src/tui/app/confirm_delete_y_starts_delete
  - functions/src/tui/app/up_recalls_previous_messages_without_sending_them
  - functions/src/tui/app/recalled_message_can_be_edited_before_resending
  - functions/src/tui/app/up_moves_the_cursor_inside_a_multiline_draft
  - functions/src/tui/app/up_is_plain_cursor_movement_when_there_is_no_history
  - functions/src/tui/app/consecutive_duplicate_submissions_are_stored_once
  - functions/src/tui/app/chat_shift_enter_inserts_newline_instead_of_submitting
  - functions/src/tui/app/chat_alt_enter_inserts_newline_as_non_kitty_fallback
  - functions/src/tui/app/chat_left_arrow_moves_cursor_for_mid_buffer_insert
  - functions/src/tui/app/chat_backspace_deletes_at_cursor_not_always_the_last_char
  - functions/src/tui/app/chat_home_and_end_move_cursor_to_line_boundaries
  - functions/src/tui/app/chat_ctrl_left_right_jump_by_word
  - functions/src/tui/app/chat_ctrl_backspace_deletes_whole_word
  - functions/src/tui/app/altgr_backslash_reaches_the_input
  - functions/src/tui/app/plain_at_sign_still_opens_the_file_picker
  - functions/src/tui/app/typed_backslashes_reach_the_input
  - functions/src/tui/app/paste_inserts_at_cursor_not_always_appended_at_the_end
  - functions/src/tui/app/shift_tab_cycles_auto_mode_through_all_three_levels_and_wraps
  - functions/src/tui/app/shift_tab_works_from_any_mode_not_just_chat
  - functions/src/tui/app/setting_auto_mode_state_shares_the_same_cell_as_a_clone
  - functions/src/tui/app/typing_and_submitting_slash_skills_opens_the_dialog_end_to_end
  - functions/src/tui/app/skills_view_up_down_navigation_clamps_at_bounds
  - functions/src/tui/app/skills_view_esc_returns_to_chat
  - functions/src/tui/app/mcp_view_up_down_navigation_clamps_at_bounds
  - functions/src/tui/app/mcp_view_esc_returns_to_chat
  - functions/src/tui/app/chat_plain_enter_submits_and_clears_buffer
  - functions/src/tui/app/chat_plain_enter_on_empty_buffer_does_nothing
  - functions/src/tui/app/ctrl_o_opens_model_info_panel_and_esc_closes_it
  - functions/src/tui/app/chat_ctrl_enter_still_submits_as_legacy_alias
  - functions/src/tui/app/provider_switch_up_down_navigation_clamps_at_bounds
  - functions/src/tui/app/provider_switch_esc_returns_to_chat
  - functions/src/tui/app/llama_cpp_up_down_navigation_clamps_at_bounds
  - functions/src/tui/app/llama_cpp_esc_returns_to_chat
  - functions/src/tui/app/llama_cpp_typing_fills_the_download_input
  - functions/src/tui/app/llama_cpp_delete_key_asks_for_confirmation_before_deleting
  - functions/src/tui/app/llama_cpp_switch_is_a_noop_while_a_delete_is_already_running
---

# Signature

`fn key(code: KeyCode) -> crossterm::event::KeyEvent`

# Calls

- [empty](../../../../functions/src/logging/LoggerGuard/empty.md)

# Called by

- [model_download_typing_filters_suggestions](../../../../functions/src/tui/app/model_download_typing_filters_suggestions.md)
- [model_download_backspace_removes_last_char](../../../../functions/src/tui/app/model_download_backspace_removes_last_char.md)
- [model_download_tab_adopts_highlighted_suggestion](../../../../functions/src/tui/app/model_download_tab_adopts_highlighted_suggestion.md)
- [model_download_esc_closes_dialog_without_running_pull](../../../../functions/src/tui/app/model_download_esc_closes_dialog_without_running_pull.md)
- [model_download_enter_starts_pull_then_esc_aborts_it](../../../../functions/src/tui/app/model_download_enter_starts_pull_then_esc_aborts_it.md)
- [delete_key_ignored_for_uninstalled_suggestion](../../../../functions/src/tui/app/delete_key_ignored_for_uninstalled_suggestion.md)
- [delete_key_on_installed_model_asks_for_confirmation](../../../../functions/src/tui/app/delete_key_on_installed_model_asks_for_confirmation.md)
- [confirm_delete_n_cancels_back_to_list](../../../../functions/src/tui/app/confirm_delete_n_cancels_back_to_list.md)
- [confirm_delete_esc_cancels_back_to_list_without_closing_dialog](../../../../functions/src/tui/app/confirm_delete_esc_cancels_back_to_list_without_closing_dialog.md)
- [confirm_delete_y_starts_delete](../../../../functions/src/tui/app/confirm_delete_y_starts_delete.md)
- [up_recalls_previous_messages_without_sending_them](../../../../functions/src/tui/app/up_recalls_previous_messages_without_sending_them.md)
- [recalled_message_can_be_edited_before_resending](../../../../functions/src/tui/app/recalled_message_can_be_edited_before_resending.md)
- [up_moves_the_cursor_inside_a_multiline_draft](../../../../functions/src/tui/app/up_moves_the_cursor_inside_a_multiline_draft.md)
- [up_is_plain_cursor_movement_when_there_is_no_history](../../../../functions/src/tui/app/up_is_plain_cursor_movement_when_there_is_no_history.md)
- [consecutive_duplicate_submissions_are_stored_once](../../../../functions/src/tui/app/consecutive_duplicate_submissions_are_stored_once.md)
- [chat_shift_enter_inserts_newline_instead_of_submitting](../../../../functions/src/tui/app/chat_shift_enter_inserts_newline_instead_of_submitting.md)
- [chat_alt_enter_inserts_newline_as_non_kitty_fallback](../../../../functions/src/tui/app/chat_alt_enter_inserts_newline_as_non_kitty_fallback.md)
- [chat_left_arrow_moves_cursor_for_mid_buffer_insert](../../../../functions/src/tui/app/chat_left_arrow_moves_cursor_for_mid_buffer_insert.md)
- [chat_backspace_deletes_at_cursor_not_always_the_last_char](../../../../functions/src/tui/app/chat_backspace_deletes_at_cursor_not_always_the_last_char.md)
- [chat_home_and_end_move_cursor_to_line_boundaries](../../../../functions/src/tui/app/chat_home_and_end_move_cursor_to_line_boundaries.md)
- [chat_ctrl_left_right_jump_by_word](../../../../functions/src/tui/app/chat_ctrl_left_right_jump_by_word.md)
- [chat_ctrl_backspace_deletes_whole_word](../../../../functions/src/tui/app/chat_ctrl_backspace_deletes_whole_word.md)
- [altgr_backslash_reaches_the_input](../../../../functions/src/tui/app/altgr_backslash_reaches_the_input.md)
- [plain_at_sign_still_opens_the_file_picker](../../../../functions/src/tui/app/plain_at_sign_still_opens_the_file_picker.md)
- [typed_backslashes_reach_the_input](../../../../functions/src/tui/app/typed_backslashes_reach_the_input.md)
- [paste_inserts_at_cursor_not_always_appended_at_the_end](../../../../functions/src/tui/app/paste_inserts_at_cursor_not_always_appended_at_the_end.md)
- [shift_tab_cycles_auto_mode_through_all_three_levels_and_wraps](../../../../functions/src/tui/app/shift_tab_cycles_auto_mode_through_all_three_levels_and_wraps.md)
- [shift_tab_works_from_any_mode_not_just_chat](../../../../functions/src/tui/app/shift_tab_works_from_any_mode_not_just_chat.md)
- [setting_auto_mode_state_shares_the_same_cell_as_a_clone](../../../../functions/src/tui/app/setting_auto_mode_state_shares_the_same_cell_as_a_clone.md)
- [typing_and_submitting_slash_skills_opens_the_dialog_end_to_end](../../../../functions/src/tui/app/typing_and_submitting_slash_skills_opens_the_dialog_end_to_end.md)
- [skills_view_up_down_navigation_clamps_at_bounds](../../../../functions/src/tui/app/skills_view_up_down_navigation_clamps_at_bounds.md)
- [skills_view_esc_returns_to_chat](../../../../functions/src/tui/app/skills_view_esc_returns_to_chat.md)
- [mcp_view_up_down_navigation_clamps_at_bounds](../../../../functions/src/tui/app/mcp_view_up_down_navigation_clamps_at_bounds.md)
- [mcp_view_esc_returns_to_chat](../../../../functions/src/tui/app/mcp_view_esc_returns_to_chat.md)
- [chat_plain_enter_submits_and_clears_buffer](../../../../functions/src/tui/app/chat_plain_enter_submits_and_clears_buffer.md)
- [chat_plain_enter_on_empty_buffer_does_nothing](../../../../functions/src/tui/app/chat_plain_enter_on_empty_buffer_does_nothing.md)
- [ctrl_o_opens_model_info_panel_and_esc_closes_it](../../../../functions/src/tui/app/ctrl_o_opens_model_info_panel_and_esc_closes_it.md)
- [chat_ctrl_enter_still_submits_as_legacy_alias](../../../../functions/src/tui/app/chat_ctrl_enter_still_submits_as_legacy_alias.md)
- [provider_switch_up_down_navigation_clamps_at_bounds](../../../../functions/src/tui/app/provider_switch_up_down_navigation_clamps_at_bounds.md)
- [provider_switch_esc_returns_to_chat](../../../../functions/src/tui/app/provider_switch_esc_returns_to_chat.md)
- [llama_cpp_up_down_navigation_clamps_at_bounds](../../../../functions/src/tui/app/llama_cpp_up_down_navigation_clamps_at_bounds.md)
- [llama_cpp_esc_returns_to_chat](../../../../functions/src/tui/app/llama_cpp_esc_returns_to_chat.md)
- [llama_cpp_typing_fills_the_download_input](../../../../functions/src/tui/app/llama_cpp_typing_fills_the_download_input.md)
- [llama_cpp_delete_key_asks_for_confirmation_before_deleting](../../../../functions/src/tui/app/llama_cpp_delete_key_asks_for_confirmation_before_deleting.md)
- [llama_cpp_switch_is_a_noop_while_a_delete_is_already_running](../../../../functions/src/tui/app/llama_cpp_switch_is_a_noop_while_a_delete_is_already_running.md)