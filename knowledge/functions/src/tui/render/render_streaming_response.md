---
type: Rust Function
title: render_streaming_response
resource: src/tui/render.rs#L487-L503
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/markdown/parse_markdown
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/render/render_chat
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn render_streaming_response(app: &App, model_name: &str) -> Vec<Line<'static>>`

# Calls

- [parse_markdown](../../../../functions/src/tui/markdown/parse_markdown.md)

# Called by

- [render_chat](../../../../functions/src/tui/render/render_chat.md)