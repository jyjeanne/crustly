---
type: Rust Function
title: approval_dialog_details_view_shows_pretty_printed_json
resource: src/tui/render.rs#L4092-L4104
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/render/test_approval_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_to_string
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn approval_dialog_details_view_shows_pretty_printed_json()`

# Calls

- [test_approval_request](../../../../functions/src/tui/render/test_approval_request.md)
- [render_to_string](../../../../functions/src/tui/render/render_to_string.md)