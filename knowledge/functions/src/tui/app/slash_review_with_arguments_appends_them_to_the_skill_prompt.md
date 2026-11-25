---
type: Rust Function
title: slash_review_with_arguments_appends_them_to_the_skill_prompt
resource: src/tui/app.rs#L4964-L4972
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/app/App/create_new_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/try_handle_slash_command
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn slash_review_with_arguments_appends_them_to_the_skill_prompt()`

# Calls

- [create_new_session](../../../../functions/src/tui/app/App/create_new_session.md)
- [try_handle_slash_command](../../../../functions/src/tui/app/App/try_handle_slash_command.md)
- [contains](../../../../functions/src/memory/vector_index/VectorIndex/contains.md)