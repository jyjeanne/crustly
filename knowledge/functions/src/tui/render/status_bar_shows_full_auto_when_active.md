---
type: Rust Function
title: status_bar_shows_full_auto_when_active
resource: src/tui/render.rs#L3311-L3327
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/app/App/set_auto_mode_state
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_to_string
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn status_bar_shows_full_auto_when_active()`

# Calls

- [set_auto_mode_state](../../../../functions/src/tui/app/App/set_auto_mode_state.md)
- [render_to_string](../../../../functions/src/tui/render/render_to_string.md)