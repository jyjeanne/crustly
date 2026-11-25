---
type: Rust Function
title: handle_ollama_delete_finished_failure_keeps_installed_and_posts_error
resource: src/tui/app.rs#L4214-L4234
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/app/App/handle_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn handle_ollama_delete_finished_failure_keeps_installed_and_posts_error()`

# Calls

- [handle_event](../../../../functions/src/tui/app/App/handle_event.md)
- [contains](../../../../functions/src/memory/vector_index/VectorIndex/contains.md)