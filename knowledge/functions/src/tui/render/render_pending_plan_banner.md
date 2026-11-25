---
type: Rust Function
title: render_pending_plan_banner
resource: src/tui/render.rs#L310-L349
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/tui/render/render_chat
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn render_pending_plan_banner(app: &App) -> Vec<Line<'static>>`

# Called by

- [render_chat](../../../../functions/src/tui/render/render_chat.md)