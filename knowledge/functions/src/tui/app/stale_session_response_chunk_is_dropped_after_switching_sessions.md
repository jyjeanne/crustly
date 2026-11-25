---
type: Rust Function
title: stale_session_response_chunk_is_dropped_after_switching_sessions
resource: src/tui/app.rs#L5680-L5716
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
  - target: functions/src/tui/app/App/handle_event
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn stale_session_response_chunk_is_dropped_after_switching_sessions()`

# Calls

- [create_new_session](../../../../functions/src/tui/app/App/create_new_session.md)
- [handle_event](../../../../functions/src/tui/app/App/handle_event.md)