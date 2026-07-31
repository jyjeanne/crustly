---
type: Rust Method
title: switch_mode
resource: src/tui/app.rs#L2064-L2073
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/tui/app/App/load_sessions
  called_by:
  - functions/src/tui/app/App/handle_event
  - functions/src/tui/app/App/handle_key_event
  - functions/src/tui/app/App/handle_sessions_key
  - functions/src/tui/app/App/open_skills
  - functions/src/tui/app/App/handle_skills_key
  - functions/src/tui/app/App/open_mcp
  - functions/src/tui/app/App/handle_mcp_key
  - functions/src/tui/app/App/try_handle_slash_command
  - functions/src/tui/app/App/handle_plan_key
  - functions/src/tui/app/App/open_file_picker
  - functions/src/tui/app/App/handle_file_picker_key
  - functions/src/tui/app/App/open_model_download
  - functions/src/tui/app/App/handle_model_download_key
  - functions/src/tui/app/App/open_provider_switch
  - functions/src/tui/app/App/handle_provider_switch_key
  - functions/src/tui/app/App/switch_provider_to_ollama_model
---

# Signature

`async fn switch_mode(&mut self, mode: AppMode) -> Result<()>`

# Calls

- [load_sessions](../../../../../functions/src/tui/app/App/load_sessions.md)

# Called by

- [handle_event](../../../../../functions/src/tui/app/App/handle_event.md)
- [handle_key_event](../../../../../functions/src/tui/app/App/handle_key_event.md)
- [handle_sessions_key](../../../../../functions/src/tui/app/App/handle_sessions_key.md)
- [open_skills](../../../../../functions/src/tui/app/App/open_skills.md)
- [handle_skills_key](../../../../../functions/src/tui/app/App/handle_skills_key.md)
- [open_mcp](../../../../../functions/src/tui/app/App/open_mcp.md)
- [handle_mcp_key](../../../../../functions/src/tui/app/App/handle_mcp_key.md)
- [try_handle_slash_command](../../../../../functions/src/tui/app/App/try_handle_slash_command.md)
- [handle_plan_key](../../../../../functions/src/tui/app/App/handle_plan_key.md)
- [open_file_picker](../../../../../functions/src/tui/app/App/open_file_picker.md)
- [handle_file_picker_key](../../../../../functions/src/tui/app/App/handle_file_picker_key.md)
- [open_model_download](../../../../../functions/src/tui/app/App/open_model_download.md)
- [handle_model_download_key](../../../../../functions/src/tui/app/App/handle_model_download_key.md)
- [open_provider_switch](../../../../../functions/src/tui/app/App/open_provider_switch.md)
- [handle_provider_switch_key](../../../../../functions/src/tui/app/App/handle_provider_switch_key.md)
- [switch_provider_to_ollama_model](../../../../../functions/src/tui/app/App/switch_provider_to_ollama_model.md)