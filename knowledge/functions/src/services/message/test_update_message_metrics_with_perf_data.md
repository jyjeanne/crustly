---
type: Rust Function
title: test_update_message_metrics_with_perf_data
resource: src/services/message.rs#L351-L384
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/services/message/MessageService/create_message
  - functions/src/services/message/MessageService/update_message_metrics
  - functions/src/services/message/MessageService/get_message_required
  - functions/src/config/secrets/SecretString/from_str
---

# Signature

`async fn test_update_message_metrics_with_perf_data()`

# Calls

- [create_message](../../../../functions/src/services/message/MessageService/create_message.md)
- [update_message_metrics](../../../../functions/src/services/message/MessageService/update_message_metrics.md)
- [get_message_required](../../../../functions/src/services/message/MessageService/get_message_required.md)
- [from_str](../../../../functions/src/config/secrets/SecretString/from_str.md)