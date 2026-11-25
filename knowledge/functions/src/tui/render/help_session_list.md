---
type: Rust Function
title: help_session_list
resource: src/tui/render.rs#L1267-L1281
visibility: private
generated:
  by: okf-rs/0.8.0
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

`fn help_session_list() -> Vec<Line<'static>>`

# Calls

- [help_section_header](../../../../functions/src/tui/render/help_section_header.md)
- [help_row](../../../../functions/src/tui/render/help_row.md)

# Called by

- [render_help](../../../../functions/src/tui/render/render_help.md)