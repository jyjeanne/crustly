---
type: Rust Function
title: render_approval_input_summary
resource: src/tui/render.rs#L1325-L1364
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/config/secrets/SecretString/len
  called_by:
  - functions/src/tui/render/render_approval
---

# Signature

`fn render_approval_input_summary(request: &super::events::ToolApprovalRequest) -> Vec<Line<'_>>`

# Calls

- [is_empty](../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [len](../../../../functions/src/config/secrets/SecretString/len.md)

# Called by

- [render_approval](../../../../functions/src/tui/render/render_approval.md)