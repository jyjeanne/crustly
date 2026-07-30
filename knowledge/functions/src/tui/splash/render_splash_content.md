---
type: Rust Function
title: render_splash_content
resource: src/tui/splash.rs#L37-L170
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/plan/PlanTask/block
  called_by:
  - functions/src/tui/splash/render_splash
---

# Signature

`fn render_splash_content(f: &mut Frame, area: Rect, provider_name: &str, model_name: &str)`

# Calls

- [block](../../../../functions/src/plan/PlanTask/block.md)

# Called by

- [render_splash](../../../../functions/src/tui/splash/render_splash.md)