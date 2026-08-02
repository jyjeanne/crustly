---
type: Rust Function
title: render_llama_cpp_download_progress
resource: src/tui/render.rs#L2147-L2203
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/plan/PlanTask/block
  called_by:
  - functions/src/tui/render/render_llama_cpp_models
---

# Signature

`fn render_llama_cpp_download_progress(f: &mut Frame, app: &App, area: Rect)`

# Calls

- [block](../../../../functions/src/plan/PlanTask/block.md)

# Called by

- [render_llama_cpp_models](../../../../functions/src/tui/render/render_llama_cpp_models.md)