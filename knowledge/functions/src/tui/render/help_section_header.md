---
type: Rust Function
title: help_section_header
resource: src/tui/render.rs#L726-L732
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/tui/render/help_global_commands
  - functions/src/tui/render/help_chat_mode
  - functions/src/tui/render/help_session_list
  - functions/src/tui/render/help_plan_mode
  - functions/src/tui/render/help_features
---

# Signature

`fn help_section_header(title: &'static str) -> [Line<'static>; 3]`

# Called by

- [help_global_commands](../../../../functions/src/tui/render/help_global_commands.md)
- [help_chat_mode](../../../../functions/src/tui/render/help_chat_mode.md)
- [help_session_list](../../../../functions/src/tui/render/help_session_list.md)
- [help_plan_mode](../../../../functions/src/tui/render/help_plan_mode.md)
- [help_features](../../../../functions/src/tui/render/help_features.md)