---
type: Rust Method
title: reload_memory_rows
resource: src/tui/app.rs#L1864-L1887
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/project/identify
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/extraction/MemoryStore/pending
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/embedding_status
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/clamp_memory_selection
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/load_memory_history
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/open_memory_view
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/approve_selected_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/forget_selected_memory
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn reload_memory_rows(&mut self)`

# Calls

- [identify](../../../../../functions/src/memory/project/identify.md)
- [pending](../../../../../functions/src/memory/extraction/MemoryStore/pending.md)
- [push](../../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)
- [embedding_status](../../../../../functions/src/memory/MemoryStore/embedding_status.md)
- [clamp_memory_selection](../../../../../functions/src/tui/app/App/clamp_memory_selection.md)
- [load_memory_history](../../../../../functions/src/tui/app/App/load_memory_history.md)

# Called by

- [open_memory_view](../../../../../functions/src/tui/app/App/open_memory_view.md)
- [approve_selected_memory](../../../../../functions/src/tui/app/App/approve_selected_memory.md)
- [forget_selected_memory](../../../../../functions/src/tui/app/App/forget_selected_memory.md)