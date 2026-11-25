---
type: Rust Function
title: switching_back_to_a_session_with_a_still_in_flight_request_restores_processing_state
resource: src/tui/app.rs#L5940-L5962
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
  - target: functions/src/tui/app/App/load_session
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn switching_back_to_a_session_with_a_still_in_flight_request_restores_processing_state()`

# Calls

- [create_new_session](../../../../functions/src/tui/app/App/create_new_session.md)
- [load_session](../../../../functions/src/tui/app/App/load_session.md)