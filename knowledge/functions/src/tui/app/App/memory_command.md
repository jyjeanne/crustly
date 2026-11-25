---
type: Rust Method
title: memory_command
resource: src/tui/app.rs#L1810-L1854
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/app/App/push_memory_note
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/identify
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/try_handle_slash_command
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn memory_command(&mut self)`

# Calls

- [push_memory_note](../../../../../functions/src/tui/app/App/push_memory_note.md)
- [identify](../../../../../functions/src/memory/project/identify.md)
- [push](../../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)

# Called by

- [try_handle_slash_command](../../../../../functions/src/tui/app/App/try_handle_slash_command.md)