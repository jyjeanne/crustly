---
type: Rust Method
title: auto_mode
resource: src/tui/app.rs#L967-L972
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/tui/app/auto_mode_defaults_to_interactive
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/shift_tab_cycles_auto_mode_through_all_three_levels_and_wraps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/shift_tab_works_from_any_mode_not_just_chat
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/setting_auto_mode_state_shares_the_same_cell_as_a_clone
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_status_bar
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn auto_mode(&self) -> PlanExecMode`

# Called by

- [auto_mode_defaults_to_interactive](../../../../../functions/src/tui/app/auto_mode_defaults_to_interactive.md)
- [shift_tab_cycles_auto_mode_through_all_three_levels_and_wraps](../../../../../functions/src/tui/app/shift_tab_cycles_auto_mode_through_all_three_levels_and_wraps.md)
- [shift_tab_works_from_any_mode_not_just_chat](../../../../../functions/src/tui/app/shift_tab_works_from_any_mode_not_just_chat.md)
- [setting_auto_mode_state_shares_the_same_cell_as_a_clone](../../../../../functions/src/tui/app/setting_auto_mode_state_shares_the_same_cell_as_a_clone.md)
- [render_status_bar](../../../../../functions/src/tui/render/render_status_bar.md)