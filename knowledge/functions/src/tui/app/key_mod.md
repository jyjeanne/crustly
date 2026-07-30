---
type: Rust Function
title: key_mod
resource: src/tui/app.rs#L3014-L3016
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/tui/app/chat_shift_enter_inserts_newline_instead_of_submitting
  - functions/src/tui/app/chat_alt_enter_inserts_newline_as_non_kitty_fallback
  - functions/src/tui/app/chat_ctrl_left_right_jump_by_word
  - functions/src/tui/app/chat_ctrl_backspace_deletes_whole_word
  - functions/src/tui/app/altgr_backslash_reaches_the_input
  - functions/src/tui/app/altgr_at_sign_is_typed_not_treated_as_the_file_picker_shortcut
  - functions/src/tui/app/ctrl_y_with_no_response_yet_shows_error_without_touching_clipboard
  - functions/src/tui/app/ctrl_y_copies_last_code_block_when_present
  - functions/src/tui/app/ctrl_v_paste_from_clipboard_fails_gracefully_without_panicking
  - functions/src/tui/app/ctrl_o_opens_model_info_panel_and_esc_closes_it
  - functions/src/tui/app/chat_ctrl_enter_still_submits_as_legacy_alias
  - functions/src/tui/app/ctrl_w_opens_provider_switch_dialog_in_loading_state
---

# Signature

`fn key_mod(code: KeyCode, modifiers: KeyModifiers) -> crossterm::event::KeyEvent`

# Called by

- [chat_shift_enter_inserts_newline_instead_of_submitting](../../../../functions/src/tui/app/chat_shift_enter_inserts_newline_instead_of_submitting.md)
- [chat_alt_enter_inserts_newline_as_non_kitty_fallback](../../../../functions/src/tui/app/chat_alt_enter_inserts_newline_as_non_kitty_fallback.md)
- [chat_ctrl_left_right_jump_by_word](../../../../functions/src/tui/app/chat_ctrl_left_right_jump_by_word.md)
- [chat_ctrl_backspace_deletes_whole_word](../../../../functions/src/tui/app/chat_ctrl_backspace_deletes_whole_word.md)
- [altgr_backslash_reaches_the_input](../../../../functions/src/tui/app/altgr_backslash_reaches_the_input.md)
- [altgr_at_sign_is_typed_not_treated_as_the_file_picker_shortcut](../../../../functions/src/tui/app/altgr_at_sign_is_typed_not_treated_as_the_file_picker_shortcut.md)
- [ctrl_y_with_no_response_yet_shows_error_without_touching_clipboard](../../../../functions/src/tui/app/ctrl_y_with_no_response_yet_shows_error_without_touching_clipboard.md)
- [ctrl_y_copies_last_code_block_when_present](../../../../functions/src/tui/app/ctrl_y_copies_last_code_block_when_present.md)
- [ctrl_v_paste_from_clipboard_fails_gracefully_without_panicking](../../../../functions/src/tui/app/ctrl_v_paste_from_clipboard_fails_gracefully_without_panicking.md)
- [ctrl_o_opens_model_info_panel_and_esc_closes_it](../../../../functions/src/tui/app/ctrl_o_opens_model_info_panel_and_esc_closes_it.md)
- [chat_ctrl_enter_still_submits_as_legacy_alias](../../../../functions/src/tui/app/chat_ctrl_enter_still_submits_as_legacy_alias.md)
- [ctrl_w_opens_provider_switch_dialog_in_loading_state](../../../../functions/src/tui/app/ctrl_w_opens_provider_switch_dialog_in_loading_state.md)