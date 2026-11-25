---
type: Rust Function
title: unrecognized_slash_word_falls_through_instead_of_being_swallowed
resource: src/tui/app.rs#L4923-L4935
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/app/App/try_handle_slash_command
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn unrecognized_slash_word_falls_through_instead_of_being_swallowed()`

# Calls

- [try_handle_slash_command](../../../../functions/src/tui/app/App/try_handle_slash_command.md)