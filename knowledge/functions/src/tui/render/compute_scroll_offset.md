---
type: Rust Function
title: compute_scroll_offset
resource: src/tui/render.rs#L429-L432
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/tui/render/render_chat
---

# Signature

`fn compute_scroll_offset(total_lines: usize, visible_height: usize, scroll_offset: usize) -> u16`

# Called by

- [render_chat](../../../../functions/src/tui/render/render_chat.md)