---
type: Rust Function
title: clear_session_is_refused_while_the_current_session_is_processing
resource: src/tui/app.rs#L5863-L5896
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
  - target: functions/src/services/message/MessageService/create_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/clear_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/MessageService/list_messages_for_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn clear_session_is_refused_while_the_current_session_is_processing()`

# Calls

- [create_new_session](../../../../functions/src/tui/app/App/create_new_session.md)
- [create_message](../../../../functions/src/services/message/MessageService/create_message.md)
- [clear_session](../../../../functions/src/tui/app/App/clear_session.md)
- [list_messages_for_session](../../../../functions/src/services/message/MessageService/list_messages_for_session.md)
- [contains](../../../../functions/src/memory/vector_index/VectorIndex/contains.md)