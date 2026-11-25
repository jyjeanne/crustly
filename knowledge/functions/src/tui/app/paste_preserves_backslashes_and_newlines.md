---
type: Rust Function
title: paste_preserves_backslashes_and_newlines
resource: src/tui/app.rs#L4483-L4497
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/app/App/handle_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/input_text
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn paste_preserves_backslashes_and_newlines()`

# Calls

- [handle_event](../../../../functions/src/tui/app/App/handle_event.md)
- [input_text](../../../../functions/src/tui/app/App/input_text.md)