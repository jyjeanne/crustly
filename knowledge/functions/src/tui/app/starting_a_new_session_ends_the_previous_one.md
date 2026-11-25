---
type: Rust Function
title: starting_a_new_session_ends_the_previous_one
resource: src/tui/app.rs#L6450-L6461
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/app/app_with_extraction
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/create_new_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/episodes
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn starting_a_new_session_ends_the_previous_one()`

# Calls

- [app_with_extraction](../../../../functions/src/tui/app/app_with_extraction.md)
- [create_new_session](../../../../functions/src/tui/app/App/create_new_session.md)
- [episodes](../../../../functions/src/tui/app/episodes.md)