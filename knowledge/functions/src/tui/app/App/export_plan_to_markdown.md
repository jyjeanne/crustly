---
type: Rust Method
title: export_plan_to_markdown
resource: src/tui/app.rs#L2058-L2135
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/tui/app/App/handle_plan_key
---

# Signature

`async fn export_plan_to_markdown(&self, filename: &str) -> Result<()>`

# Calls

- [is_empty](../../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [handle_plan_key](../../../../../functions/src/tui/app/App/handle_plan_key.md)