---
type: Rust Function
title: render_streaming_response
resource: src/tui/render.rs#L381-L397
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/tui/markdown/parse_markdown
  called_by:
  - functions/src/tui/render/render_chat
---

# Signature

`fn render_streaming_response(app: &App, model_name: &str) -> Vec<Line<'static>>`

# Calls

- [parse_markdown](../../../../functions/src/tui/markdown/parse_markdown.md)

# Called by

- [render_chat](../../../../functions/src/tui/render/render_chat.md)