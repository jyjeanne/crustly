---
type: Rust Function
title: render_help
resource: src/tui/render.rs#L916-L951
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/tui/render/help_global_commands
  - functions/src/tui/render/help_chat_mode
  - functions/src/tui/render/help_session_list
  - functions/src/tui/render/help_plan_mode
  - functions/src/tui/render/help_features
  - functions/src/tui/render/help_footer
  - functions/src/plan/PlanTask/block
  called_by:
  - functions/src/tui/render/render
---

# Signature

`fn render_help(f: &mut Frame, app: &App, area: Rect)`

# Calls

- [help_global_commands](../../../../functions/src/tui/render/help_global_commands.md)
- [help_chat_mode](../../../../functions/src/tui/render/help_chat_mode.md)
- [help_session_list](../../../../functions/src/tui/render/help_session_list.md)
- [help_plan_mode](../../../../functions/src/tui/render/help_plan_mode.md)
- [help_features](../../../../functions/src/tui/render/help_features.md)
- [help_footer](../../../../functions/src/tui/render/help_footer.md)
- [block](../../../../functions/src/plan/PlanTask/block.md)

# Called by

- [render](../../../../functions/src/tui/render/render.md)