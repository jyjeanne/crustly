---
type: Rust Function
title: llama_cpp_models_dialog_shows_display_name_not_the_raw_path
resource: src/tui/render.rs#L3649-L3659
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/render/llama_cpp_model_fixture
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_to_string
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn llama_cpp_models_dialog_shows_display_name_not_the_raw_path()`

# Calls

- [llama_cpp_model_fixture](../../../../functions/src/tui/render/llama_cpp_model_fixture.md)
- [render_to_string](../../../../functions/src/tui/render/render_to_string.md)