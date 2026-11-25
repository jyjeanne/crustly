---
type: Rust Function
title: test_approval_request
resource: src/tui/render.rs#L4055-L4070
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/tui/render/approval_dialog_shows_tool_name_capabilities_and_summarized_params
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/approval_dialog_details_view_shows_pretty_printed_json
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_approval_request( tool_input: serde_json::Value, capabilities: Vec<String>, ) -> crate::tui::events::ToolApprovalRequest`

# Called by

- [approval_dialog_shows_tool_name_capabilities_and_summarized_params](../../../../functions/src/tui/render/approval_dialog_shows_tool_name_capabilities_and_summarized_params.md)
- [approval_dialog_details_view_shows_pretty_printed_json](../../../../functions/src/tui/render/approval_dialog_details_view_shows_pretty_printed_json.md)