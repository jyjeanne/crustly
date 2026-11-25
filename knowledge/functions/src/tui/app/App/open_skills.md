---
type: Rust Method
title: open_skills
resource: src/tui/app.rs#L1665-L1669
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/skill/list_skills
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/switch_mode
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/try_handle_slash_command
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn open_skills(&mut self) -> Result<()>`

# Calls

- [list_skills](../../../../../functions/src/llm/tools/skill/list_skills.md)
- [switch_mode](../../../../../functions/src/tui/app/App/switch_mode.md)

# Called by

- [try_handle_slash_command](../../../../../functions/src/tui/app/App/try_handle_slash_command.md)