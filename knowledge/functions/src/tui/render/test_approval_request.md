---
type: Rust Function
title: test_approval_request
resource: src/tui/render.rs#L2989-L3003
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/tui/render/approval_dialog_shows_tool_name_capabilities_and_summarized_params
  - functions/src/tui/render/approval_dialog_details_view_shows_pretty_printed_json
---

# Signature

`fn test_approval_request( tool_input: serde_json::Value, capabilities: Vec<String>, ) -> crate::tui::events::ToolApprovalRequest`

# Called by

- [approval_dialog_shows_tool_name_capabilities_and_summarized_params](../../../../functions/src/tui/render/approval_dialog_shows_tool_name_capabilities_and_summarized_params.md)
- [approval_dialog_details_view_shows_pretty_printed_json](../../../../functions/src/tui/render/approval_dialog_details_view_shows_pretty_printed_json.md)