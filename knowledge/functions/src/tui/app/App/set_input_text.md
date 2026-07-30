---
type: Rust Method
title: set_input_text
resource: src/tui/app.rs#L314-L317
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/tui/app/plain_textarea
  called_by:
  - functions/src/tui/app/App/load_history_entry
  - functions/src/tui/app/App/handle_plan_key
  - functions/src/tui/app/up_recalls_previous_messages_without_sending_them
  - functions/src/tui/app/up_moves_the_cursor_inside_a_multiline_draft
  - functions/src/tui/app/up_is_plain_cursor_movement_when_there_is_no_history
  - functions/src/tui/app/chat_input_text_is_not_underlined
---

# Signature

`fn set_input_text(&mut self, text: &str)`

# Calls

- [plain_textarea](../../../../../functions/src/tui/app/plain_textarea.md)

# Called by

- [load_history_entry](../../../../../functions/src/tui/app/App/load_history_entry.md)
- [handle_plan_key](../../../../../functions/src/tui/app/App/handle_plan_key.md)
- [up_recalls_previous_messages_without_sending_them](../../../../../functions/src/tui/app/up_recalls_previous_messages_without_sending_them.md)
- [up_moves_the_cursor_inside_a_multiline_draft](../../../../../functions/src/tui/app/up_moves_the_cursor_inside_a_multiline_draft.md)
- [up_is_plain_cursor_movement_when_there_is_no_history](../../../../../functions/src/tui/app/up_is_plain_cursor_movement_when_there_is_no_history.md)
- [chat_input_text_is_not_underlined](../../../../../functions/src/tui/app/chat_input_text_is_not_underlined.md)