---
type: Rust Method
title: set_auto_mode_state
resource: src/tui/app.rs#L490-L492
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/cli/cmd_chat
  - functions/src/tui/app/setting_auto_mode_state_shares_the_same_cell_as_a_clone
  - functions/src/tui/render/status_bar_shows_full_auto_when_active
---

# Signature

`pub fn set_auto_mode_state(&mut self, auto_mode: Arc<Mutex<PlanExecMode>>)`

# Called by

- [cmd_chat](../../../../../functions/src/cli/cmd_chat.md)
- [setting_auto_mode_state_shares_the_same_cell_as_a_clone](../../../../../functions/src/tui/app/setting_auto_mode_state_shares_the_same_cell_as_a_clone.md)
- [status_bar_shows_full_auto_when_active](../../../../../functions/src/tui/render/status_bar_shows_full_auto_when_active.md)