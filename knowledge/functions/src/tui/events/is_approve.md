---
type: Rust Function
title: is_approve
resource: src/tui/events.rs#L427-L432
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/tui/app/App/handle_approval_key
---

# Signature

`pub fn is_approve(event: &KeyEvent) -> bool`

# Calls

- [is_empty](../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [handle_approval_key](../../../../functions/src/tui/app/App/handle_approval_key.md)