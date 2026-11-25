---
type: Rust Function
title: handle_ollama_pull_finished_failure_posts_error_message
resource: src/tui/app.rs#L4079-L4094
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
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

`async fn handle_ollama_pull_finished_failure_posts_error_message()`

# Calls

- [handle_event](../../../../functions/src/tui/app/App/handle_event.md)
- [contains](../../../../functions/src/memory/vector_index/VectorIndex/contains.md)