---
type: Rust Method
title: handle_skills_key
resource: src/tui/app.rs#L1672-L1684
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/events/is_cancel
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/switch_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/is_up
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/is_down
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/handle_key_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/skills_view_up_down_navigation_clamps_at_bounds
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/skills_view_esc_returns_to_chat
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn handle_skills_key(&mut self, event: crossterm::event::KeyEvent) -> Result<()>`

# Calls

- [is_cancel](../../../../../functions/src/tui/events/is_cancel.md)
- [switch_mode](../../../../../functions/src/tui/app/App/switch_mode.md)
- [is_up](../../../../../functions/src/tui/events/is_up.md)
- [is_down](../../../../../functions/src/tui/events/is_down.md)

# Called by

- [handle_key_event](../../../../../functions/src/tui/app/App/handle_key_event.md)
- [skills_view_up_down_navigation_clamps_at_bounds](../../../../../functions/src/tui/app/skills_view_up_down_navigation_clamps_at_bounds.md)
- [skills_view_esc_returns_to_chat](../../../../../functions/src/tui/app/skills_view_esc_returns_to_chat.md)