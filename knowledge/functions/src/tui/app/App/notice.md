---
type: Rust Method
title: notice
resource: src/tui/app.rs#L741-L746
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/tui/app/App/handle_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/a_save_during_a_response_is_announced_in_the_chat
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/ctrl_y_copies_last_code_block_when_present
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/ctrl_c_clears_the_input_first_and_quits_on_a_second_press
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/ctrl_c_quits_only_if_the_second_press_comes_soon
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/a_multi_line_paste_is_inserted_whole_without_sending
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_status_bar
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn notice(&self) -> Option<&str>`

# Called by

- [handle_event](../../../../../functions/src/tui/app/App/handle_event.md)
- [a_save_during_a_response_is_announced_in_the_chat](../../../../../functions/src/tui/app/a_save_during_a_response_is_announced_in_the_chat.md)
- [ctrl_y_copies_last_code_block_when_present](../../../../../functions/src/tui/app/ctrl_y_copies_last_code_block_when_present.md)
- [ctrl_c_clears_the_input_first_and_quits_on_a_second_press](../../../../../functions/src/tui/app/ctrl_c_clears_the_input_first_and_quits_on_a_second_press.md)
- [ctrl_c_quits_only_if_the_second_press_comes_soon](../../../../../functions/src/tui/app/ctrl_c_quits_only_if_the_second_press_comes_soon.md)
- [a_multi_line_paste_is_inserted_whole_without_sending](../../../../../functions/src/tui/app/a_multi_line_paste_is_inserted_whole_without_sending.md)
- [render_status_bar](../../../../../functions/src/tui/render/render_status_bar.md)