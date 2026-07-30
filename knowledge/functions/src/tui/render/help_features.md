---
type: Rust Function
title: help_features
resource: src/tui/render.rs#L864-L884
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/tui/render/help_section_header
  - functions/src/tui/render/feature_row
  called_by:
  - functions/src/tui/render/render_help
---

# Signature

`fn help_features() -> Vec<Line<'static>>`

# Calls

- [help_section_header](../../../../functions/src/tui/render/help_section_header.md)
- [feature_row](../../../../functions/src/tui/render/feature_row.md)

# Called by

- [render_help](../../../../functions/src/tui/render/render_help.md)