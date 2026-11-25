---
type: Rust Function
title: plural
resource: src/tui/app.rs#L338-L344
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/tui/app/App/copy_last_response_to_clipboard
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/paste_text
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn plural(count: usize, noun: &str) -> String`

# Called by

- [copy_last_response_to_clipboard](../../../../functions/src/tui/app/App/copy_last_response_to_clipboard.md)
- [paste_text](../../../../functions/src/tui/app/App/paste_text.md)