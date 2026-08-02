---
type: Rust Function
title: render_llama_cpp_models
resource: src/tui/render.rs#L2012-L2144
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/tui/render/render_llama_cpp_confirm_delete
  - functions/src/tui/render/render_llama_cpp_deleting
  - functions/src/tui/render/render_llama_cpp_switching
  - functions/src/tui/render/render_llama_cpp_download_progress
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/plan/PlanTask/block
  called_by:
  - functions/src/tui/render/render
---

# Signature

`fn render_llama_cpp_models(f: &mut Frame, app: &App, area: Rect)`

# Calls

- [render_llama_cpp_confirm_delete](../../../../functions/src/tui/render/render_llama_cpp_confirm_delete.md)
- [render_llama_cpp_deleting](../../../../functions/src/tui/render/render_llama_cpp_deleting.md)
- [render_llama_cpp_switching](../../../../functions/src/tui/render/render_llama_cpp_switching.md)
- [render_llama_cpp_download_progress](../../../../functions/src/tui/render/render_llama_cpp_download_progress.md)
- [is_empty](../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [block](../../../../functions/src/plan/PlanTask/block.md)

# Called by

- [render](../../../../functions/src/tui/render/render.md)