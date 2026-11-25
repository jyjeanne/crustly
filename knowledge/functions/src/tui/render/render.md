---
type: Rust Function
title: render
resource: src/tui/render.rs#L23-L91
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/render/input_box_height
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_header
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_chat
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_input
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_plan
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_plan_help
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_sessions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_help
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_settings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_approval
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_file_picker
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_model_download
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_model_info
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_provider_switch
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_llama_cpp_models
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_skills
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_mcp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_status_bar
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/render/render_to_string
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/runner/run_loop
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn render(f: &mut Frame, app: &App)`

# Calls

- [input_box_height](../../../../functions/src/tui/render/input_box_height.md)
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
- [render_memory](../../../../functions/src/tui/render/render_memory.md)
- [render_status_bar](../../../../functions/src/tui/render/render_status_bar.md)

# Called by

- [render_to_string](../../../../functions/src/tui/render/render_to_string.md)
- [run_loop](../../../../functions/src/tui/runner/run_loop.md)