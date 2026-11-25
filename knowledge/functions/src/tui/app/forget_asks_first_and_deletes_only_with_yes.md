---
type: Rust Function
title: forget_asks_first_and_deletes_only_with_yes
resource: src/tui/app.rs#L6070-L6093
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
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
  - target: functions/src/tui/app/last_note
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn forget_asks_first_and_deletes_only_with_yes()`

# Calls

- [memory_test_app](../../../../functions/src/tui/app/memory_test_app.md)
- [try_handle_slash_command](../../../../functions/src/tui/app/App/try_handle_slash_command.md)
- [identify](../../../../functions/src/memory/project/identify.md)
- [last_note](../../../../functions/src/tui/app/last_note.md)