---
type: Rust Method
title: open_mcp
resource: src/tui/app.rs#L2064-L2067
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/app/App/switch_mode
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/try_handle_slash_command
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn open_mcp(&mut self) -> Result<()>`

# Calls

- [switch_mode](../../../../../functions/src/tui/app/App/switch_mode.md)

# Called by

- [try_handle_slash_command](../../../../../functions/src/tui/app/App/try_handle_slash_command.md)