---
type: Rust Function
title: test_calculate_totals
resource: src/services/message.rs#L559-L595
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/services/session/SessionService/create_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/MessageService/create_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/MessageService/update_message_usage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/MessageService/calculate_total_tokens
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/MessageService/calculate_total_cost
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_calculate_totals()`

# Calls

- [create_session](../../../../functions/src/services/session/SessionService/create_session.md)
- [create_message](../../../../functions/src/services/message/MessageService/create_message.md)
- [update_message_usage](../../../../functions/src/services/message/MessageService/update_message_usage.md)
- [calculate_total_tokens](../../../../functions/src/services/message/MessageService/calculate_total_tokens.md)
- [calculate_total_cost](../../../../functions/src/services/message/MessageService/calculate_total_cost.md)