---
type: Rust Function
title: test_calculate_totals
resource: src/services/message.rs#L544-L580
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/services/message/MessageService/create_message
  - functions/src/services/message/MessageService/update_message_usage
  - functions/src/services/message/MessageService/calculate_total_tokens
  - functions/src/services/message/MessageService/calculate_total_cost
---

# Signature

`async fn test_calculate_totals()`

# Calls

- [create_message](../../../../functions/src/services/message/MessageService/create_message.md)
- [update_message_usage](../../../../functions/src/services/message/MessageService/update_message_usage.md)
- [calculate_total_tokens](../../../../functions/src/services/message/MessageService/calculate_total_tokens.md)
- [calculate_total_cost](../../../../functions/src/services/message/MessageService/calculate_total_cost.md)