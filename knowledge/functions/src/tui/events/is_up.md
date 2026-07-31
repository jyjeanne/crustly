---
type: Rust Function
title: is_up
resource: src/tui/events.rs#L370-L372
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/tui/app/App/handle_sessions_key
  - functions/src/tui/app/App/handle_skills_key
  - functions/src/tui/app/App/handle_mcp_key
  - functions/src/tui/app/App/handle_file_picker_key
  - functions/src/tui/app/App/handle_model_download_key
  - functions/src/tui/app/App/handle_provider_switch_key
---

# Signature

`pub fn is_up(event: &KeyEvent) -> bool`

# Calls

- [is_empty](../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [handle_sessions_key](../../../../functions/src/tui/app/App/handle_sessions_key.md)
- [handle_skills_key](../../../../functions/src/tui/app/App/handle_skills_key.md)
- [handle_mcp_key](../../../../functions/src/tui/app/App/handle_mcp_key.md)
- [handle_file_picker_key](../../../../functions/src/tui/app/App/handle_file_picker_key.md)
- [handle_model_download_key](../../../../functions/src/tui/app/App/handle_model_download_key.md)
- [handle_provider_switch_key](../../../../functions/src/tui/app/App/handle_provider_switch_key.md)