---
type: Rust Function
title: render_llama_cpp_switching
resource: src/tui/render.rs#L2287-L2321
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

`fn render_llama_cpp_switching(f: &mut Frame, path: &std::path::Path, area: Rect)`

# Calls

- [block](../../../../functions/src/plan/PlanTask/block.md)

# Called by

- [render_llama_cpp_models](../../../../functions/src/tui/render/render_llama_cpp_models.md)