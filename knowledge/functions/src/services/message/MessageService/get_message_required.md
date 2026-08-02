---
type: Rust Method
title: get_message_required
resource: src/services/message.rs#L69-L73
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/services/message/MessageService/get_message
  called_by:
  - functions/src/services/message/MessageService/update_message_usage
  - functions/src/services/message/MessageService/update_message_metrics
  - functions/src/services/message/test_update_message_usage
  - functions/src/services/message/test_update_message_metrics_with_perf_data
  - functions/src/services/message/test_update_message_metrics_without_perf_data
---

# Signature

`pub async fn get_message_required(&self, id: Uuid) -> Result<Message>`

# Calls

- [get_message](../../../../../functions/src/services/message/MessageService/get_message.md)

# Called by

- [update_message_usage](../../../../../functions/src/services/message/MessageService/update_message_usage.md)
- [update_message_metrics](../../../../../functions/src/services/message/MessageService/update_message_metrics.md)
- [test_update_message_usage](../../../../../functions/src/services/message/test_update_message_usage.md)
- [test_update_message_metrics_with_perf_data](../../../../../functions/src/services/message/test_update_message_metrics_with_perf_data.md)
- [test_update_message_metrics_without_perf_data](../../../../../functions/src/services/message/test_update_message_metrics_without_perf_data.md)