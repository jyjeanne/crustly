---
type: Rust Function
title: slash_review_resolves_to_the_builtin_skill
resource: src/tui/app.rs#L4942-L4959
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
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

`async fn slash_review_resolves_to_the_builtin_skill()`

# Calls

- [create_new_session](../../../../functions/src/tui/app/App/create_new_session.md)
- [try_handle_slash_command](../../../../functions/src/tui/app/App/try_handle_slash_command.md)
- [contains](../../../../functions/src/memory/vector_index/VectorIndex/contains.md)