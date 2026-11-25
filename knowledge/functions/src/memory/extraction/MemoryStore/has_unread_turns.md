---
type: Rust Method
title: has_unread_turns
resource: src/memory/extraction.rs#L84-L88
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/MemoryRepository/extraction_state
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/turns_after
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/finish_memory
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn has_unread_turns(&self, session: Uuid) -> Result<bool>`

# Calls

- [extraction_state](../../../../../functions/src/db/repository/memory/MemoryRepository/extraction_state.md)
- [turns_after](../../../../../functions/src/db/repository/memory/MemoryRepository/turns_after.md)

# Called by

- [finish_memory](../../../../../functions/src/tui/app/App/finish_memory.md)