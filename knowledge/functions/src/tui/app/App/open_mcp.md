---
type: Rust Method
title: open_mcp
resource: src/tui/app.rs#L1021-L1024
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/tui/app/App/switch_mode
  called_by:
  - functions/src/tui/app/App/try_handle_slash_command
---

# Signature

`async fn open_mcp(&mut self) -> Result<()>`

# Calls

- [switch_mode](../../../../../functions/src/tui/app/App/switch_mode.md)

# Called by

- [try_handle_slash_command](../../../../../functions/src/tui/app/App/try_handle_slash_command.md)