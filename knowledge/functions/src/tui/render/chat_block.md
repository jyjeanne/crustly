---
type: Rust Function
title: chat_block
resource: src/tui/render.rs#L556-L571
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/tui/render/render_chat
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_welcome
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn chat_block(scrolled_up: bool) -> Block<'static>`

# Called by

- [render_chat](../../../../functions/src/tui/render/render_chat.md)
- [render_welcome](../../../../functions/src/tui/render/render_welcome.md)