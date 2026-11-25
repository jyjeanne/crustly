---
type: Rust Function
title: remember_without_text_shows_usage_and_saves_nothing
resource: src/tui/app.rs#L6060-L6067
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
  - target: functions/src/memory/project/identify
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/try_handle_slash_command
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/last_note
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn remember_without_text_shows_usage_and_saves_nothing()`

# Calls

- [memory_test_app](../../../../functions/src/tui/app/memory_test_app.md)
- [identify](../../../../functions/src/memory/project/identify.md)
- [try_handle_slash_command](../../../../functions/src/tui/app/App/try_handle_slash_command.md)
- [last_note](../../../../functions/src/tui/app/last_note.md)