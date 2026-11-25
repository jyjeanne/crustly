---
type: Rust Function
title: plain_textarea
resource: src/tui/app.rs#L357-L361
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/tui/app/App/new
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/clear_input
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/set_input_text
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn plain_textarea() -> TextArea<'static>`

# Called by

- [new](../../../../functions/src/tui/app/App/new.md)
- [clear_input](../../../../functions/src/tui/app/App/clear_input.md)
- [set_input_text](../../../../functions/src/tui/app/App/set_input_text.md)