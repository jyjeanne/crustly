---
type: Rust Method
title: end_session_in_background
resource: src/tui/app.rs#L830-L839
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/project/identify
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/extraction/MemoryStore/trigger
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/create_new_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/load_session
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn end_session_in_background(&self, next: Option<Uuid>)`

# Calls

- [identify](../../../../../functions/src/memory/project/identify.md)
- [trigger](../../../../../functions/src/memory/extraction/MemoryStore/trigger.md)

# Called by

- [create_new_session](../../../../../functions/src/tui/app/App/create_new_session.md)
- [load_session](../../../../../functions/src/tui/app/App/load_session.md)