---
type: Rust Function
title: is_enter
resource: src/tui/events.rs#L365-L367
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/tui/app/App/handle_sessions_key
  - functions/src/tui/app/App/handle_file_picker_key
  - functions/src/tui/app/App/handle_model_download_key
  - functions/src/tui/app/App/handle_provider_switch_key
---

# Signature

`pub fn is_enter(event: &KeyEvent) -> bool`

# Calls

- [is_empty](../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [handle_sessions_key](../../../../functions/src/tui/app/App/handle_sessions_key.md)
- [handle_file_picker_key](../../../../functions/src/tui/app/App/handle_file_picker_key.md)
- [handle_model_download_key](../../../../functions/src/tui/app/App/handle_model_download_key.md)
- [handle_provider_switch_key](../../../../functions/src/tui/app/App/handle_provider_switch_key.md)