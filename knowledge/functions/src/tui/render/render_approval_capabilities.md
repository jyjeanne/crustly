---
type: Rust Function
title: render_approval_capabilities
resource: src/tui/render.rs#L1291-L1308
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/tui/render/render_approval
---

# Signature

`fn render_approval_capabilities(request: &super::events::ToolApprovalRequest) -> Vec<Line<'_>>`

# Calls

- [is_empty](../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [render_approval](../../../../functions/src/tui/render/render_approval.md)