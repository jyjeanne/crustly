---
type: Rust Function
title: test_update_message_metrics_with_perf_data
resource: src/services/message.rs#L366-L399
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/services/session/SessionService/create_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/MessageService/create_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/MessageService/update_message_metrics
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/MessageService/get_message_required
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_update_message_metrics_with_perf_data()`

# Calls

- [create_session](../../../../functions/src/services/session/SessionService/create_session.md)
- [create_message](../../../../functions/src/services/message/MessageService/create_message.md)
- [update_message_metrics](../../../../functions/src/services/message/MessageService/update_message_metrics.md)
- [get_message_required](../../../../functions/src/services/message/MessageService/get_message_required.md)