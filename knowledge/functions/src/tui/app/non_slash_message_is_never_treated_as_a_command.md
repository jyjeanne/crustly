---
type: Rust Function
title: non_slash_message_is_never_treated_as_a_command
resource: src/tui/app.rs#L5025-L5032
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/app/App/try_handle_slash_command
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn non_slash_message_is_never_treated_as_a_command()`

# Calls

- [try_handle_slash_command](../../../../functions/src/tui/app/App/try_handle_slash_command.md)