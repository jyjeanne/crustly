---
type: Rust Method
title: handle_skills_key
resource: src/tui/app.rs#L1268-L1280
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/tui/events/is_cancel
  - functions/src/tui/app/App/switch_mode
  - functions/src/tui/events/is_up
  - functions/src/tui/events/is_down
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/config/secrets/SecretString/len
  called_by:
  - functions/src/tui/app/App/handle_key_event
  - functions/src/tui/app/skills_view_up_down_navigation_clamps_at_bounds
  - functions/src/tui/app/skills_view_esc_returns_to_chat
---

# Signature

`async fn handle_skills_key(&mut self, event: crossterm::event::KeyEvent) -> Result<()>`

# Calls

- [is_cancel](../../../../../functions/src/tui/events/is_cancel.md)
- [switch_mode](../../../../../functions/src/tui/app/App/switch_mode.md)
- [is_up](../../../../../functions/src/tui/events/is_up.md)
- [is_down](../../../../../functions/src/tui/events/is_down.md)
- [is_empty](../../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [len](../../../../../functions/src/config/secrets/SecretString/len.md)

# Called by

- [handle_key_event](../../../../../functions/src/tui/app/App/handle_key_event.md)
- [skills_view_up_down_navigation_clamps_at_bounds](../../../../../functions/src/tui/app/skills_view_up_down_navigation_clamps_at_bounds.md)
- [skills_view_esc_returns_to_chat](../../../../../functions/src/tui/app/skills_view_esc_returns_to_chat.md)