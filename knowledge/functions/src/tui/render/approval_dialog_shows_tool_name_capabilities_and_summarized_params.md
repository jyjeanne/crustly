---
type: Rust Function
title: approval_dialog_shows_tool_name_capabilities_and_summarized_params
resource: src/tui/render.rs#L4073-L4089
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

`async fn approval_dialog_shows_tool_name_capabilities_and_summarized_params()`

# Calls

- [test_approval_request](../../../../functions/src/tui/render/test_approval_request.md)
- [render_to_string](../../../../functions/src/tui/render/render_to_string.md)