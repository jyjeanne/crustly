---
type: Rust Function
title: paste_with_embedded_newline_produces_multiple_lines
resource: src/tui/app.rs#L4586-L4595
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
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

`async fn paste_with_embedded_newline_produces_multiple_lines()`

# Calls

- [handle_event](../../../../functions/src/tui/app/App/handle_event.md)
- [input_text](../../../../functions/src/tui/app/App/input_text.md)