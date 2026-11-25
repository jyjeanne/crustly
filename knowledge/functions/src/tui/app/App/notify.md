---
type: Rust Method
title: notify
resource: src/tui/app.rs#L736-L738
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/tui/app/App/copy_last_response_to_clipboard
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/App/paste_text
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/App/handle_ctrl_c
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/App/announce_memory_saves
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn notify(&mut self, text: impl Into<String>)`

# Called by

- [copy_last_response_to_clipboard](../../../../../functions/src/tui/app/App/copy_last_response_to_clipboard.md)
- [paste_text](../../../../../functions/src/tui/app/App/paste_text.md)
- [handle_ctrl_c](../../../../../functions/src/tui/app/App/handle_ctrl_c.md)
- [announce_memory_saves](../../../../../functions/src/tui/app/App/announce_memory_saves.md)