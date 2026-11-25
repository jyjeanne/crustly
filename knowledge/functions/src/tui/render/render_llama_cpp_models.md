---
type: Rust Function
title: render_llama_cpp_models
resource: src/tui/render.rs#L2762-L2883
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/render/render_llama_cpp_confirm_delete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_llama_cpp_deleting
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_llama_cpp_switching
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_llama_cpp_download_progress
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/llama_cpp_hardware_line
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/llama_cpp_model_lines
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/render/render
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn render_llama_cpp_models(f: &mut Frame, app: &App, area: Rect)`

# Calls

- [render_llama_cpp_confirm_delete](../../../../functions/src/tui/render/render_llama_cpp_confirm_delete.md)
- [render_llama_cpp_deleting](../../../../functions/src/tui/render/render_llama_cpp_deleting.md)
- [render_llama_cpp_switching](../../../../functions/src/tui/render/render_llama_cpp_switching.md)
- [render_llama_cpp_download_progress](../../../../functions/src/tui/render/render_llama_cpp_download_progress.md)
- [llama_cpp_hardware_line](../../../../functions/src/tui/render/llama_cpp_hardware_line.md)
- [llama_cpp_model_lines](../../../../functions/src/tui/render/llama_cpp_model_lines.md)

# Called by

- [render](../../../../functions/src/tui/render/render.md)