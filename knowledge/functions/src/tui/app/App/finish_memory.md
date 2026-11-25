---
type: Rust Method
title: finish_memory
resource: src/tui/app.rs#L845-L871
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/extraction/MemoryStore/extraction_on
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/extraction/MemoryStore/has_unread_turns
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/identify
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/extraction/MemoryStore/finish_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/take_notices
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/quitting_writes_the_session_episode_before_exit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/runner/run
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn finish_memory(&self)`

# Calls

- [extraction_on](../../../../../functions/src/memory/extraction/MemoryStore/extraction_on.md)
- [has_unread_turns](../../../../../functions/src/memory/extraction/MemoryStore/has_unread_turns.md)
- [identify](../../../../../functions/src/memory/project/identify.md)
- [finish_session](../../../../../functions/src/memory/extraction/MemoryStore/finish_session.md)
- [take_notices](../../../../../functions/src/memory/MemoryStore/take_notices.md)

# Called by

- [quitting_writes_the_session_episode_before_exit](../../../../../functions/src/tui/app/quitting_writes_the_session_episode_before_exit.md)
- [run](../../../../../functions/src/tui/runner/run.md)