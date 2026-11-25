---
type: Rust Method
title: paste_from_clipboard
resource: src/tui/app.rs#L686-L693
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/app/App/clipboard
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/paste_text
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/handle_chat_key
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn paste_from_clipboard(&mut self)`

# Calls

- [clipboard](../../../../../functions/src/tui/app/App/clipboard.md)
- [paste_text](../../../../../functions/src/tui/app/App/paste_text.md)

# Called by

- [handle_chat_key](../../../../../functions/src/tui/app/App/handle_chat_key.md)