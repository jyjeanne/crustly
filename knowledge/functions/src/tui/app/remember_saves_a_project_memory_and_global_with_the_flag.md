---
type: Rust Function
title: remember_saves_a_project_memory_and_global_with_the_flag
resource: src/tui/app.rs#L6041-L6057
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
---

# Signature

`async fn remember_saves_a_project_memory_and_global_with_the_flag()`

# Calls

- [memory_test_app](../../../../functions/src/tui/app/memory_test_app.md)
- [identify](../../../../functions/src/memory/project/identify.md)
- [try_handle_slash_command](../../../../functions/src/tui/app/App/try_handle_slash_command.md)