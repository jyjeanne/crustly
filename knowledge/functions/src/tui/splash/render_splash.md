---
type: Rust Function
title: render_splash
resource: src/tui/splash.rs#L14-L35
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/tui/splash/render_splash_content
  called_by:
  - functions/src/tui/render/render
---

# Signature

`pub fn render_splash(f: &mut Frame, area: Rect, provider_name: &str, model_name: &str)`

# Calls

- [render_splash_content](../../../../functions/src/tui/splash/render_splash_content.md)

# Called by

- [render](../../../../functions/src/tui/render/render.md)