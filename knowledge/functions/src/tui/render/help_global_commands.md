---
type: Rust Function
title: help_global_commands
resource: src/tui/render.rs#L738-L778
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/tui/render/help_section_header
  - functions/src/tui/render/help_row
  called_by:
  - functions/src/tui/render/render_help
---

# Signature

`fn help_global_commands() -> Vec<Line<'static>>`

# Calls

- [help_section_header](../../../../functions/src/tui/render/help_section_header.md)
- [help_row](../../../../functions/src/tui/render/help_row.md)

# Called by

- [render_help](../../../../functions/src/tui/render/render_help.md)