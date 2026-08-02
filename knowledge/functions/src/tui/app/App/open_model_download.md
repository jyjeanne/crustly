---
type: Rust Method
title: open_model_download
resource: src/tui/app.rs#L2494-L2512
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/tui/app/App/refresh_model_download_suggestions
  - functions/src/tui/app/App/event_sender
  - functions/src/tui/app/App/switch_mode
  called_by:
  - functions/src/tui/app/App/handle_key_event
  - functions/src/tui/app/open_model_download_switches_mode_and_seeds_suggestions
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
---

# Signature

`async fn open_model_download(&mut self) -> Result<()>`

# Calls

- [refresh_model_download_suggestions](../../../../../functions/src/tui/app/App/refresh_model_download_suggestions.md)
- [event_sender](../../../../../functions/src/tui/app/App/event_sender.md)
- [switch_mode](../../../../../functions/src/tui/app/App/switch_mode.md)

# Called by

- [handle_key_event](../../../../../functions/src/tui/app/App/handle_key_event.md)
- [open_model_download_switches_mode_and_seeds_suggestions](../../../../../functions/src/tui/app/open_model_download_switches_mode_and_seeds_suggestions.md)
- [model_download_typing_filters_suggestions](../../../../../functions/src/tui/app/model_download_typing_filters_suggestions.md)
- [model_download_backspace_removes_last_char](../../../../../functions/src/tui/app/model_download_backspace_removes_last_char.md)
- [model_download_tab_adopts_highlighted_suggestion](../../../../../functions/src/tui/app/model_download_tab_adopts_highlighted_suggestion.md)
- [model_download_esc_closes_dialog_without_running_pull](../../../../../functions/src/tui/app/model_download_esc_closes_dialog_without_running_pull.md)
- [model_download_enter_starts_pull_then_esc_aborts_it](../../../../../functions/src/tui/app/model_download_enter_starts_pull_then_esc_aborts_it.md)
- [delete_key_ignored_for_uninstalled_suggestion](../../../../../functions/src/tui/app/delete_key_ignored_for_uninstalled_suggestion.md)
- [delete_key_on_installed_model_asks_for_confirmation](../../../../../functions/src/tui/app/delete_key_on_installed_model_asks_for_confirmation.md)
- [confirm_delete_n_cancels_back_to_list](../../../../../functions/src/tui/app/confirm_delete_n_cancels_back_to_list.md)
- [confirm_delete_esc_cancels_back_to_list_without_closing_dialog](../../../../../functions/src/tui/app/confirm_delete_esc_cancels_back_to_list_without_closing_dialog.md)
- [confirm_delete_y_starts_delete](../../../../../functions/src/tui/app/confirm_delete_y_starts_delete.md)