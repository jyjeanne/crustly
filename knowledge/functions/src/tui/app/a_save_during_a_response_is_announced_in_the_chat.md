---
type: Rust Function
title: a_save_during_a_response_is_announced_in_the_chat
resource: src/tui/app.rs#L6121-L6151
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/app/memory_test_app
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/try_handle_slash_command
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/identify
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/take_notices
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/announce_memory_saves
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/notice
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn a_save_during_a_response_is_announced_in_the_chat()`

# Calls

- [memory_test_app](../../../../functions/src/tui/app/memory_test_app.md)
- [try_handle_slash_command](../../../../functions/src/tui/app/App/try_handle_slash_command.md)
- [identify](../../../../functions/src/memory/project/identify.md)
- [take_notices](../../../../functions/src/memory/MemoryStore/take_notices.md)
- [announce_memory_saves](../../../../functions/src/tui/app/App/announce_memory_saves.md)
- [notice](../../../../functions/src/tui/app/App/notice.md)
- [contains](../../../../functions/src/memory/vector_index/VectorIndex/contains.md)