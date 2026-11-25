---
type: Rust Function
title: help_chat_mode
resource: src/tui/render.rs#L1189-L1265
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/render/help_section_header
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/help_row
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/render/render_help
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn help_chat_mode(app: &App) -> Vec<Line<'static>>`

# Calls

- [help_section_header](../../../../functions/src/tui/render/help_section_header.md)
- [help_row](../../../../functions/src/tui/render/help_row.md)

# Called by

- [render_help](../../../../functions/src/tui/render/render_help.md)