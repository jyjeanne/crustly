---
type: Rust Method
title: push_input_history
resource: src/tui/app.rs#L387-L394
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/tui/app/App/handle_chat_key
  - functions/src/tui/app/up_recalls_previous_messages_without_sending_them
  - functions/src/tui/app/key_release_events_are_ignored
  - functions/src/tui/app/recalled_message_can_be_edited_before_resending
  - functions/src/tui/app/up_moves_the_cursor_inside_a_multiline_draft
  - functions/src/tui/app/consecutive_duplicate_submissions_are_stored_once
---

# Signature

`fn push_input_history(&mut self, content: &str)`

# Called by

- [handle_chat_key](../../../../../functions/src/tui/app/App/handle_chat_key.md)
- [up_recalls_previous_messages_without_sending_them](../../../../../functions/src/tui/app/up_recalls_previous_messages_without_sending_them.md)
- [key_release_events_are_ignored](../../../../../functions/src/tui/app/key_release_events_are_ignored.md)
- [recalled_message_can_be_edited_before_resending](../../../../../functions/src/tui/app/recalled_message_can_be_edited_before_resending.md)
- [up_moves_the_cursor_inside_a_multiline_draft](../../../../../functions/src/tui/app/up_moves_the_cursor_inside_a_multiline_draft.md)
- [consecutive_duplicate_submissions_are_stored_once](../../../../../functions/src/tui/app/consecutive_duplicate_submissions_are_stored_once.md)