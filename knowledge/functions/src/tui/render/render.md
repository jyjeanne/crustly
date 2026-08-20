---
type: Rust Function
title: render
resource: src/tui/render.rs#L19-L88
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/tui/render/render_header
  - functions/src/tui/render/render_chat
  - functions/src/tui/render/render_input
  - functions/src/tui/render/render_plan
  - functions/src/tui/render/render_plan_help
  - functions/src/tui/render/render_sessions
  - functions/src/tui/render/render_help
  - functions/src/tui/render/render_settings
  - functions/src/tui/render/render_approval
  - functions/src/tui/render/render_file_picker
  - functions/src/tui/render/render_model_download
  - functions/src/tui/render/render_model_info
  - functions/src/tui/render/render_provider_switch
  - functions/src/tui/render/render_llama_cpp_models
  - functions/src/tui/render/render_skills
  - functions/src/tui/render/render_mcp
  - functions/src/tui/render/render_status_bar
  called_by:
  - functions/src/tui/render/render_to_string
  - functions/src/tui/runner/run_loop
---

# Signature

`pub fn render(f: &mut Frame, app: &App)`

# Calls

- [render_header](../../../../functions/src/tui/render/render_header.md)
- [render_chat](../../../../functions/src/tui/render/render_chat.md)
- [render_input](../../../../functions/src/tui/render/render_input.md)
- [render_plan](../../../../functions/src/tui/render/render_plan.md)
- [render_plan_help](../../../../functions/src/tui/render/render_plan_help.md)
- [render_sessions](../../../../functions/src/tui/render/render_sessions.md)
- [render_help](../../../../functions/src/tui/render/render_help.md)
- [render_settings](../../../../functions/src/tui/render/render_settings.md)
- [render_approval](../../../../functions/src/tui/render/render_approval.md)
- [render_file_picker](../../../../functions/src/tui/render/render_file_picker.md)
- [render_model_download](../../../../functions/src/tui/render/render_model_download.md)
- [render_model_info](../../../../functions/src/tui/render/render_model_info.md)
- [render_provider_switch](../../../../functions/src/tui/render/render_provider_switch.md)
- [render_llama_cpp_models](../../../../functions/src/tui/render/render_llama_cpp_models.md)
- [render_skills](../../../../functions/src/tui/render/render_skills.md)
- [render_mcp](../../../../functions/src/tui/render/render_mcp.md)
- [render_status_bar](../../../../functions/src/tui/render/render_status_bar.md)

# Called by

- [render_to_string](../../../../functions/src/tui/render/render_to_string.md)
- [run_loop](../../../../functions/src/tui/runner/run_loop.md)