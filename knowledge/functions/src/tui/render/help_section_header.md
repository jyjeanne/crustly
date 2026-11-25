---
type: Rust Function
title: help_section_header
resource: src/tui/render.rs#L1135-L1141
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
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
---

# Signature

`fn help_section_header(title: &'static str) -> [Line<'static>; 3]`

# Called by

- [help_global_commands](../../../../functions/src/tui/render/help_global_commands.md)
- [help_chat_mode](../../../../functions/src/tui/render/help_chat_mode.md)
- [help_session_list](../../../../functions/src/tui/render/help_session_list.md)
- [help_plan_mode](../../../../functions/src/tui/render/help_plan_mode.md)
- [help_features](../../../../functions/src/tui/render/help_features.md)
- [help_about](../../../../functions/src/tui/render/help_about.md)