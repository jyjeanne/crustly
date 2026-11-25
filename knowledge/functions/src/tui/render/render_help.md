---
type: Rust Function
title: render_help
resource: src/tui/render.rs#L1381-L1425
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/render/help_global_commands
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/help_chat_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/help_session_list
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/help_plan_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/help_features
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/help_about
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/help_footer
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanTask/block
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/render/render
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn render_help(f: &mut Frame, app: &App, area: Rect)`

# Calls

- [help_global_commands](../../../../functions/src/tui/render/help_global_commands.md)
- [help_chat_mode](../../../../functions/src/tui/render/help_chat_mode.md)
- [help_session_list](../../../../functions/src/tui/render/help_session_list.md)
- [help_plan_mode](../../../../functions/src/tui/render/help_plan_mode.md)
- [help_features](../../../../functions/src/tui/render/help_features.md)
- [help_about](../../../../functions/src/tui/render/help_about.md)
- [help_footer](../../../../functions/src/tui/render/help_footer.md)
- [block](../../../../functions/src/plan/PlanTask/block.md)

# Called by

- [render](../../../../functions/src/tui/render/render.md)