---
type: Rust Method
title: try_handle_slash_command
resource: src/tui/app.rs#L1311-L1332
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/tui/events/EventHandler/next
  - functions/src/tui/app/App/open_skills
  - functions/src/tui/app/App/open_mcp
  - functions/src/tui/app/App/switch_mode
  called_by:
  - functions/src/tui/app/App/handle_chat_key
  - functions/src/tui/app/unrecognized_slash_word_falls_through_instead_of_being_swallowed
  - functions/src/tui/app/non_slash_message_is_never_treated_as_a_command
---

# Signature

`async fn try_handle_slash_command(&mut self, content: &str) -> Result<bool>`

# Calls

- [next](../../../../../functions/src/tui/events/EventHandler/next.md)
- [open_skills](../../../../../functions/src/tui/app/App/open_skills.md)
- [open_mcp](../../../../../functions/src/tui/app/App/open_mcp.md)
- [switch_mode](../../../../../functions/src/tui/app/App/switch_mode.md)

# Called by

- [handle_chat_key](../../../../../functions/src/tui/app/App/handle_chat_key.md)
- [unrecognized_slash_word_falls_through_instead_of_being_swallowed](../../../../../functions/src/tui/app/unrecognized_slash_word_falls_through_instead_of_being_swallowed.md)
- [non_slash_message_is_never_treated_as_a_command](../../../../../functions/src/tui/app/non_slash_message_is_never_treated_as_a_command.md)