---
type: Rust Method
title: clipboard
resource: src/tui/app.rs#L696-L701
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/tui/app/App/copy_last_response_to_clipboard
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/paste_from_clipboard
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn clipboard(&mut self) -> std::result::Result<&mut arboard::Clipboard, arboard::Error>`

# Called by

- [copy_last_response_to_clipboard](../../../../../functions/src/tui/app/App/copy_last_response_to_clipboard.md)
- [paste_from_clipboard](../../../../../functions/src/tui/app/App/paste_from_clipboard.md)