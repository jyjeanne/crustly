---
type: Rust Function
title: tail_chars
resource: src/tui/render.rs#L102-L113
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/tui/render/render_header
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/long_paths_are_cut_on_a_character_boundary
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn tail_chars(text: &str, max_chars: usize) -> String`

# Called by

- [render_header](../../../../functions/src/tui/render/render_header.md)
- [long_paths_are_cut_on_a_character_boundary](../../../../functions/src/tui/render/long_paths_are_cut_on_a_character_boundary.md)