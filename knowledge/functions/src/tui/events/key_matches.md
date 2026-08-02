---
type: Rust Function
title: key_matches
resource: src/tui/events.rs#L296-L298
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/tui/events/is_quit
  - functions/src/tui/events/is_new_session
  - functions/src/tui/events/is_list_sessions
  - functions/src/tui/events/is_help
  - functions/src/tui/events/is_clear_session
  - functions/src/tui/events/is_toggle_plan
  - functions/src/tui/events/is_model_download
  - functions/src/tui/events/is_model_info
  - functions/src/tui/events/is_provider_switch
  - functions/src/tui/events/is_llama_cpp_models
  - functions/src/tui/events/is_copy_response
  - functions/src/tui/events/is_paste_clipboard
---

# Signature

`pub fn key_matches(event: &KeyEvent, code: KeyCode, modifiers: KeyModifiers) -> bool`

# Called by

- [is_quit](../../../../functions/src/tui/events/is_quit.md)
- [is_new_session](../../../../functions/src/tui/events/is_new_session.md)
- [is_list_sessions](../../../../functions/src/tui/events/is_list_sessions.md)
- [is_help](../../../../functions/src/tui/events/is_help.md)
- [is_clear_session](../../../../functions/src/tui/events/is_clear_session.md)
- [is_toggle_plan](../../../../functions/src/tui/events/is_toggle_plan.md)
- [is_model_download](../../../../functions/src/tui/events/is_model_download.md)
- [is_model_info](../../../../functions/src/tui/events/is_model_info.md)
- [is_provider_switch](../../../../functions/src/tui/events/is_provider_switch.md)
- [is_llama_cpp_models](../../../../functions/src/tui/events/is_llama_cpp_models.md)
- [is_copy_response](../../../../functions/src/tui/events/is_copy_response.md)
- [is_paste_clipboard](../../../../functions/src/tui/events/is_paste_clipboard.md)