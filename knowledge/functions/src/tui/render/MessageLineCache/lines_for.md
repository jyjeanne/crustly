---
type: Rust Method
title: lines_for
resource: src/tui/render.rs#L127-L138
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/render/render_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_message_lines
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/render/render_chat
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn lines_for(&self, msg: &DisplayMessage, model_name: &str) -> Vec<Line<'static>>`

# Calls

- [render_key](../../../../../functions/src/tui/render/render_key.md)
- [render_message_lines](../../../../../functions/src/tui/render/render_message_lines.md)

# Called by

- [render_chat](../../../../../functions/src/tui/render/render_chat.md)