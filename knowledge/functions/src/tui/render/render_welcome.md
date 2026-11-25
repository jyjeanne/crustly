---
type: Rust Function
title: render_welcome
resource: src/tui/render.rs#L653-L714
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/components/ferris/walking_lines
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/chat_block
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/render/render_chat
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn render_welcome(f: &mut Frame, app: &App, area: Rect)`

# Calls

- [walking_lines](../../../../functions/src/tui/components/ferris/walking_lines.md)
- [chat_block](../../../../functions/src/tui/render/chat_block.md)

# Called by

- [render_chat](../../../../functions/src/tui/render/render_chat.md)