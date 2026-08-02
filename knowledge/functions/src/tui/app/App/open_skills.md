---
type: Rust Method
title: open_skills
resource: src/tui/app.rs#L1261-L1265
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/tools/skill/list_skills
  - functions/src/tui/app/App/switch_mode
  called_by:
  - functions/src/tui/app/App/try_handle_slash_command
---

# Signature

`async fn open_skills(&mut self) -> Result<()>`

# Calls

- [list_skills](../../../../../functions/src/llm/tools/skill/list_skills.md)
- [switch_mode](../../../../../functions/src/tui/app/App/switch_mode.md)

# Called by

- [try_handle_slash_command](../../../../../functions/src/tui/app/App/try_handle_slash_command.md)