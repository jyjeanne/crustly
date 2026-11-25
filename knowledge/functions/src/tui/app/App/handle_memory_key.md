---
type: Rust Method
title: handle_memory_key
resource: src/tui/app.rs#L1917-L2028
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/app/App/clamp_memory_selection
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/load_memory_history
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/forget_selected_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/is_cancel
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/switch_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/visible_memories
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/selected_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/approve_selected_memory
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/handle_key_event
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn handle_memory_key(&mut self, event: crossterm::event::KeyEvent) -> Result<()>`

# Calls

- [clamp_memory_selection](../../../../../functions/src/tui/app/App/clamp_memory_selection.md)
- [load_memory_history](../../../../../functions/src/tui/app/App/load_memory_history.md)
- [push](../../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)
- [forget_selected_memory](../../../../../functions/src/tui/app/App/forget_selected_memory.md)
- [is_cancel](../../../../../functions/src/tui/events/is_cancel.md)
- [switch_mode](../../../../../functions/src/tui/app/App/switch_mode.md)
- [visible_memories](../../../../../functions/src/tui/app/visible_memories.md)
- [selected_memory](../../../../../functions/src/tui/app/App/selected_memory.md)
- [approve_selected_memory](../../../../../functions/src/tui/app/App/approve_selected_memory.md)

# Called by

- [handle_key_event](../../../../../functions/src/tui/app/App/handle_key_event.md)