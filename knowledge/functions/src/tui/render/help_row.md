---
type: Rust Function
title: help_row
resource: src/tui/render.rs#L700-L709
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/tui/render/help_global_commands
  - functions/src/tui/render/help_chat_mode
  - functions/src/tui/render/help_session_list
  - functions/src/tui/render/help_plan_mode
---

# Signature

`fn help_row(key: &'static str, desc: impl Into<String>, key_color: Color) -> Line<'static>`

# Called by

- [help_global_commands](../../../../functions/src/tui/render/help_global_commands.md)
- [help_chat_mode](../../../../functions/src/tui/render/help_chat_mode.md)
- [help_session_list](../../../../functions/src/tui/render/help_session_list.md)
- [help_plan_mode](../../../../functions/src/tui/render/help_plan_mode.md)