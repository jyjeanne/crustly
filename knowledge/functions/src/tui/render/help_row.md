---
type: Rust Function
title: help_row
resource: src/tui/render.rs#L1109-L1118
visibility: private
generated:
  by: okf-rs/0.7.0
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
---

# Signature

`fn help_row(key: &'static str, desc: impl Into<String>, key_color: Color) -> Line<'static>`

# Called by

- [help_global_commands](../../../../functions/src/tui/render/help_global_commands.md)
- [help_chat_mode](../../../../functions/src/tui/render/help_chat_mode.md)
- [help_session_list](../../../../functions/src/tui/render/help_session_list.md)
- [help_plan_mode](../../../../functions/src/tui/render/help_plan_mode.md)