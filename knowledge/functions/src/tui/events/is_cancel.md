---
type: Rust Function
title: is_cancel
resource: src/tui/events.rs#L541-L543
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/tui/app/App/handle_key_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_chat_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_sessions_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_skills_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_memory_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_mcp_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_plan_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_approval_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_file_picker_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_model_download_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_provider_switch_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_llama_cpp_models_key
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn is_cancel(event: &KeyEvent) -> bool`

# Called by

- [handle_key_event](../../../../functions/src/tui/app/App/handle_key_event.md)
- [handle_chat_key](../../../../functions/src/tui/app/App/handle_chat_key.md)
- [handle_sessions_key](../../../../functions/src/tui/app/App/handle_sessions_key.md)
- [handle_skills_key](../../../../functions/src/tui/app/App/handle_skills_key.md)
- [handle_memory_key](../../../../functions/src/tui/app/App/handle_memory_key.md)
- [handle_mcp_key](../../../../functions/src/tui/app/App/handle_mcp_key.md)
- [handle_plan_key](../../../../functions/src/tui/app/App/handle_plan_key.md)
- [handle_approval_key](../../../../functions/src/tui/app/App/handle_approval_key.md)
- [handle_file_picker_key](../../../../functions/src/tui/app/App/handle_file_picker_key.md)
- [handle_model_download_key](../../../../functions/src/tui/app/App/handle_model_download_key.md)
- [handle_provider_switch_key](../../../../functions/src/tui/app/App/handle_provider_switch_key.md)
- [handle_llama_cpp_models_key](../../../../functions/src/tui/app/App/handle_llama_cpp_models_key.md)