---
type: Rust Function
title: render_approval_capabilities
resource: src/tui/render.rs#L1282-L1299
visibility: private
generated:
  by: okf-rs/0.2.0
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