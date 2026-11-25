---
type: Rust Method
title: handle_llama_cpp_models_key
resource: src/tui/app.rs#L3714-L3803
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/app/App/start_llama_cpp_delete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/is_cancel
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/switch_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/is_up
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/is_down
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/is_enter
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/start_llama_cpp_download
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/start_llama_cpp_switch
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/handle_key_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/llama_cpp_up_down_navigation_clamps_at_bounds
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/llama_cpp_esc_returns_to_chat
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/llama_cpp_typing_fills_the_download_input
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/llama_cpp_delete_key_asks_for_confirmation_before_deleting
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/llama_cpp_switch_is_a_noop_while_a_delete_is_already_running
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn handle_llama_cpp_models_key( &mut self, event: crossterm::event::KeyEvent, ) -> Result<()>`

# Calls

- [start_llama_cpp_delete](../../../../../functions/src/tui/app/App/start_llama_cpp_delete.md)
- [is_cancel](../../../../../functions/src/tui/events/is_cancel.md)
- [switch_mode](../../../../../functions/src/tui/app/App/switch_mode.md)
- [is_up](../../../../../functions/src/tui/events/is_up.md)
- [is_down](../../../../../functions/src/tui/events/is_down.md)
- [is_enter](../../../../../functions/src/tui/events/is_enter.md)
- [start_llama_cpp_download](../../../../../functions/src/tui/app/App/start_llama_cpp_download.md)
- [start_llama_cpp_switch](../../../../../functions/src/tui/app/App/start_llama_cpp_switch.md)

# Called by

- [handle_key_event](../../../../../functions/src/tui/app/App/handle_key_event.md)
- [llama_cpp_up_down_navigation_clamps_at_bounds](../../../../../functions/src/tui/app/llama_cpp_up_down_navigation_clamps_at_bounds.md)
- [llama_cpp_esc_returns_to_chat](../../../../../functions/src/tui/app/llama_cpp_esc_returns_to_chat.md)
- [llama_cpp_typing_fills_the_download_input](../../../../../functions/src/tui/app/llama_cpp_typing_fills_the_download_input.md)
- [llama_cpp_delete_key_asks_for_confirmation_before_deleting](../../../../../functions/src/tui/app/llama_cpp_delete_key_asks_for_confirmation_before_deleting.md)
- [llama_cpp_switch_is_a_noop_while_a_delete_is_already_running](../../../../../functions/src/tui/app/llama_cpp_switch_is_a_noop_while_a_delete_is_already_running.md)