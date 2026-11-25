---
type: Rust Function
title: render_model_info
resource: src/tui/render.rs#L2062-L2241
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/app/App/provider_context_window
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/App/context_usage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/llama_cpp_model_details
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/last_assistant_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/provider_icon
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/provider_name
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/App/provider_model
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/render/context_usage_color
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/render/render
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn render_model_info(f: &mut Frame, app: &App, area: Rect)`

# Calls

- [provider_context_window](../../../../functions/src/tui/app/App/provider_context_window.md)
- [context_usage](../../../../functions/src/tui/app/App/context_usage.md)
- [llama_cpp_model_details](../../../../functions/src/tui/app/App/llama_cpp_model_details.md)
- [last_assistant_message](../../../../functions/src/tui/app/App/last_assistant_message.md)
- [provider_icon](../../../../functions/src/tui/render/provider_icon.md)
- [provider_name](../../../../functions/src/tui/app/App/provider_name.md)
- [provider_model](../../../../functions/src/tui/app/App/provider_model.md)
- [context_usage_color](../../../../functions/src/tui/render/context_usage_color.md)

# Called by

- [render](../../../../functions/src/tui/render/render.md)