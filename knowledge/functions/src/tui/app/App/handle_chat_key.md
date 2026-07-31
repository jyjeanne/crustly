---
type: Rust Method
title: handle_chat_key
resource: src/tui/app.rs#L861-L969
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/tui/events/is_submit
  - functions/src/tui/app/App/input_is_blank
  - functions/src/tui/app/App/input_text
  - functions/src/tui/app/App/clear_input
  - functions/src/tui/app/App/push_input_history
  - functions/src/tui/app/App/try_handle_slash_command
  - functions/src/tui/events/is_newline
  - functions/src/tui/events/is_cancel
  - functions/src/tui/events/is_page_up
  - functions/src/tui/events/is_page_down
  - functions/src/tui/events/is_copy_response
  - functions/src/tui/app/App/copy_last_response_to_clipboard
  - functions/src/tui/events/is_paste_clipboard
  - functions/src/tui/app/App/paste_from_clipboard
  - functions/src/tui/app/App/open_file_picker
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/tui/app/App/cursor_on_first_line
  - functions/src/tui/app/App/history_prev
  - functions/src/tui/app/App/cursor_on_last_line
  - functions/src/tui/app/App/history_next
  called_by:
  - functions/src/tui/app/App/handle_key_event
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
  - functions/src/tui/app/altgr_at_sign_is_typed_not_treated_as_the_file_picker_shortcut
  - functions/src/tui/app/plain_at_sign_still_opens_the_file_picker
  - functions/src/tui/app/typed_backslashes_reach_the_input
  - functions/src/tui/app/paste_inserts_at_cursor_not_always_appended_at_the_end
  - functions/src/tui/app/ctrl_y_with_no_response_yet_shows_error_without_touching_clipboard
  - functions/src/tui/app/ctrl_y_copies_last_code_block_when_present
  - functions/src/tui/app/ctrl_v_paste_from_clipboard_fails_gracefully_without_panicking
  - functions/src/tui/app/typing_and_submitting_slash_skills_opens_the_dialog_end_to_end
  - functions/src/tui/app/chat_plain_enter_submits_and_clears_buffer
  - functions/src/tui/app/chat_plain_enter_on_empty_buffer_does_nothing
  - functions/src/tui/app/chat_ctrl_enter_still_submits_as_legacy_alias
---

# Signature

`async fn handle_chat_key(&mut self, event: crossterm::event::KeyEvent) -> Result<()>`

# Calls

- [is_submit](../../../../../functions/src/tui/events/is_submit.md)
- [input_is_blank](../../../../../functions/src/tui/app/App/input_is_blank.md)
- [input_text](../../../../../functions/src/tui/app/App/input_text.md)
- [clear_input](../../../../../functions/src/tui/app/App/clear_input.md)
- [push_input_history](../../../../../functions/src/tui/app/App/push_input_history.md)
- [try_handle_slash_command](../../../../../functions/src/tui/app/App/try_handle_slash_command.md)
- [is_newline](../../../../../functions/src/tui/events/is_newline.md)
- [is_cancel](../../../../../functions/src/tui/events/is_cancel.md)
- [is_page_up](../../../../../functions/src/tui/events/is_page_up.md)
- [is_page_down](../../../../../functions/src/tui/events/is_page_down.md)
- [is_copy_response](../../../../../functions/src/tui/events/is_copy_response.md)
- [copy_last_response_to_clipboard](../../../../../functions/src/tui/app/App/copy_last_response_to_clipboard.md)
- [is_paste_clipboard](../../../../../functions/src/tui/events/is_paste_clipboard.md)
- [paste_from_clipboard](../../../../../functions/src/tui/app/App/paste_from_clipboard.md)
- [open_file_picker](../../../../../functions/src/tui/app/App/open_file_picker.md)
- [is_empty](../../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [cursor_on_first_line](../../../../../functions/src/tui/app/App/cursor_on_first_line.md)
- [history_prev](../../../../../functions/src/tui/app/App/history_prev.md)
- [cursor_on_last_line](../../../../../functions/src/tui/app/App/cursor_on_last_line.md)
- [history_next](../../../../../functions/src/tui/app/App/history_next.md)

# Called by

- [handle_key_event](../../../../../functions/src/tui/app/App/handle_key_event.md)
- [up_recalls_previous_messages_without_sending_them](../../../../../functions/src/tui/app/up_recalls_previous_messages_without_sending_them.md)
- [recalled_message_can_be_edited_before_resending](../../../../../functions/src/tui/app/recalled_message_can_be_edited_before_resending.md)
- [up_moves_the_cursor_inside_a_multiline_draft](../../../../../functions/src/tui/app/up_moves_the_cursor_inside_a_multiline_draft.md)
- [up_is_plain_cursor_movement_when_there_is_no_history](../../../../../functions/src/tui/app/up_is_plain_cursor_movement_when_there_is_no_history.md)
- [consecutive_duplicate_submissions_are_stored_once](../../../../../functions/src/tui/app/consecutive_duplicate_submissions_are_stored_once.md)
- [chat_shift_enter_inserts_newline_instead_of_submitting](../../../../../functions/src/tui/app/chat_shift_enter_inserts_newline_instead_of_submitting.md)
- [chat_alt_enter_inserts_newline_as_non_kitty_fallback](../../../../../functions/src/tui/app/chat_alt_enter_inserts_newline_as_non_kitty_fallback.md)
- [chat_left_arrow_moves_cursor_for_mid_buffer_insert](../../../../../functions/src/tui/app/chat_left_arrow_moves_cursor_for_mid_buffer_insert.md)
- [chat_backspace_deletes_at_cursor_not_always_the_last_char](../../../../../functions/src/tui/app/chat_backspace_deletes_at_cursor_not_always_the_last_char.md)
- [chat_home_and_end_move_cursor_to_line_boundaries](../../../../../functions/src/tui/app/chat_home_and_end_move_cursor_to_line_boundaries.md)
- [chat_ctrl_left_right_jump_by_word](../../../../../functions/src/tui/app/chat_ctrl_left_right_jump_by_word.md)
- [chat_ctrl_backspace_deletes_whole_word](../../../../../functions/src/tui/app/chat_ctrl_backspace_deletes_whole_word.md)
- [altgr_backslash_reaches_the_input](../../../../../functions/src/tui/app/altgr_backslash_reaches_the_input.md)
- [altgr_at_sign_is_typed_not_treated_as_the_file_picker_shortcut](../../../../../functions/src/tui/app/altgr_at_sign_is_typed_not_treated_as_the_file_picker_shortcut.md)
- [plain_at_sign_still_opens_the_file_picker](../../../../../functions/src/tui/app/plain_at_sign_still_opens_the_file_picker.md)
- [typed_backslashes_reach_the_input](../../../../../functions/src/tui/app/typed_backslashes_reach_the_input.md)
- [paste_inserts_at_cursor_not_always_appended_at_the_end](../../../../../functions/src/tui/app/paste_inserts_at_cursor_not_always_appended_at_the_end.md)
- [ctrl_y_with_no_response_yet_shows_error_without_touching_clipboard](../../../../../functions/src/tui/app/ctrl_y_with_no_response_yet_shows_error_without_touching_clipboard.md)
- [ctrl_y_copies_last_code_block_when_present](../../../../../functions/src/tui/app/ctrl_y_copies_last_code_block_when_present.md)
- [ctrl_v_paste_from_clipboard_fails_gracefully_without_panicking](../../../../../functions/src/tui/app/ctrl_v_paste_from_clipboard_fails_gracefully_without_panicking.md)
- [typing_and_submitting_slash_skills_opens_the_dialog_end_to_end](../../../../../functions/src/tui/app/typing_and_submitting_slash_skills_opens_the_dialog_end_to_end.md)
- [chat_plain_enter_submits_and_clears_buffer](../../../../../functions/src/tui/app/chat_plain_enter_submits_and_clears_buffer.md)
- [chat_plain_enter_on_empty_buffer_does_nothing](../../../../../functions/src/tui/app/chat_plain_enter_on_empty_buffer_does_nothing.md)
- [chat_ctrl_enter_still_submits_as_legacy_alias](../../../../../functions/src/tui/app/chat_ctrl_enter_still_submits_as_legacy_alias.md)