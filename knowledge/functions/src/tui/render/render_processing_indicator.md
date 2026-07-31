---
type: Rust Function
title: render_processing_indicator
resource: src/tui/render.rs#L397-L420
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/len
  called_by:
  - functions/src/tui/render/render_chat
---

# Signature

`fn render_processing_indicator(app: &App, model_name: &str) -> Vec<Line<'static>>`

# Calls

- [len](../../../../functions/src/config/secrets/SecretString/len.md)

# Called by

- [render_chat](../../../../functions/src/tui/render/render_chat.md)