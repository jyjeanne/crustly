---
type: Rust Method
title: input_text
resource: src/tui/app.rs#L550-L552
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/tui/app/App/history_prev
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_chat_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/up_recalls_previous_messages_without_sending_them
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/key_release_events_are_ignored
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/recalled_message_can_be_edited_before_resending
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/up_moves_the_cursor_inside_a_multiline_draft
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/up_is_plain_cursor_movement_when_there_is_no_history
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/consecutive_duplicate_submissions_are_stored_once
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/chat_shift_enter_inserts_newline_instead_of_submitting
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/chat_alt_enter_inserts_newline_as_non_kitty_fallback
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/chat_left_arrow_moves_cursor_for_mid_buffer_insert
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/chat_backspace_deletes_at_cursor_not_always_the_last_char
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/chat_ctrl_backspace_deletes_whole_word
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/paste_preserves_backslashes_and_newlines
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/altgr_backslash_reaches_the_input
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/altgr_at_sign_is_typed_not_treated_as_the_file_picker_shortcut
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/typed_backslashes_reach_the_input
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/paste_inserts_at_cursor_not_always_appended_at_the_end
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/paste_with_embedded_newline_produces_multiple_lines
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/a_multi_line_paste_is_inserted_whole_without_sending
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/ctrl_z_undoes_a_paste_and_ctrl_shift_z_redoes_it
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn input_text(&self) -> String`

# Called by

- [history_prev](../../../../../functions/src/tui/app/App/history_prev.md)
- [handle_chat_key](../../../../../functions/src/tui/app/App/handle_chat_key.md)
- [up_recalls_previous_messages_without_sending_them](../../../../../functions/src/tui/app/up_recalls_previous_messages_without_sending_them.md)
- [key_release_events_are_ignored](../../../../../functions/src/tui/app/key_release_events_are_ignored.md)
- [recalled_message_can_be_edited_before_resending](../../../../../functions/src/tui/app/recalled_message_can_be_edited_before_resending.md)
- [up_moves_the_cursor_inside_a_multiline_draft](../../../../../functions/src/tui/app/up_moves_the_cursor_inside_a_multiline_draft.md)
- [up_is_plain_cursor_movement_when_there_is_no_history](../../../../../functions/src/tui/app/up_is_plain_cursor_movement_when_there_is_no_history.md)
- [consecutive_duplicate_submissions_are_stored_once](../../../../../functions/src/tui/app/consecutive_duplicate_submissions_are_stored_once.md)
- [chat_shift_enter_inserts_newline_instead_of_submitting](../../../../../functions/src/tui/app/chat_shift_enter_inserts_newline_instead_of_submitting.md)
- [chat_alt_enter_inserts_newline_as_non_kitty_fallback](../../../../../functions/src/tui/app/chat_alt_enter_inserts_newline_as_non_kitty_fallback.md)
- [chat_left_arrow_moves_cursor_for_mid_buffer_insert](../../../../../functions/src/tui/app/chat_left_arrow_moves_cursor_for_mid_buffer_insert.md)
- [chat_backspace_deletes_at_cursor_not_always_the_last_char](../../../../../functions/src/tui/app/chat_backspace_deletes_at_cursor_not_always_the_last_char.md)
- [chat_ctrl_backspace_deletes_whole_word](../../../../../functions/src/tui/app/chat_ctrl_backspace_deletes_whole_word.md)
- [paste_preserves_backslashes_and_newlines](../../../../../functions/src/tui/app/paste_preserves_backslashes_and_newlines.md)
- [altgr_backslash_reaches_the_input](../../../../../functions/src/tui/app/altgr_backslash_reaches_the_input.md)
- [altgr_at_sign_is_typed_not_treated_as_the_file_picker_shortcut](../../../../../functions/src/tui/app/altgr_at_sign_is_typed_not_treated_as_the_file_picker_shortcut.md)
- [typed_backslashes_reach_the_input](../../../../../functions/src/tui/app/typed_backslashes_reach_the_input.md)
- [paste_inserts_at_cursor_not_always_appended_at_the_end](../../../../../functions/src/tui/app/paste_inserts_at_cursor_not_always_appended_at_the_end.md)
- [paste_with_embedded_newline_produces_multiple_lines](../../../../../functions/src/tui/app/paste_with_embedded_newline_produces_multiple_lines.md)
- [a_multi_line_paste_is_inserted_whole_without_sending](../../../../../functions/src/tui/app/a_multi_line_paste_is_inserted_whole_without_sending.md)
- [ctrl_z_undoes_a_paste_and_ctrl_shift_z_redoes_it](../../../../../functions/src/tui/app/ctrl_z_undoes_a_paste_and_ctrl_shift_z_redoes_it.md)