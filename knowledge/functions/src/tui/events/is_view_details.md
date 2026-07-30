---
type: Rust Function
title: is_view_details
resource: src/tui/events.rs#L406-L408
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/tui/app/App/handle_approval_key
---

# Signature

`pub fn is_view_details(event: &KeyEvent) -> bool`

# Calls

- [is_empty](../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [handle_approval_key](../../../../functions/src/tui/app/App/handle_approval_key.md)