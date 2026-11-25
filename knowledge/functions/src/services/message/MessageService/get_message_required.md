---
type: Rust Method
title: get_message_required
resource: src/services/message.rs#L70-L74
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/services/message/MessageService/get_message
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/services/message/MessageService/update_message_usage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/MessageService/update_message_tool_log
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/MessageService/update_message_metrics
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/test_update_message_usage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/test_update_message_metrics_with_perf_data
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/test_update_message_metrics_without_perf_data
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn get_message_required(&self, id: Uuid) -> Result<Message>`

# Calls

- [get_message](../../../../../functions/src/services/message/MessageService/get_message.md)

# Called by

- [update_message_usage](../../../../../functions/src/services/message/MessageService/update_message_usage.md)
- [update_message_tool_log](../../../../../functions/src/services/message/MessageService/update_message_tool_log.md)
- [update_message_metrics](../../../../../functions/src/services/message/MessageService/update_message_metrics.md)
- [test_update_message_usage](../../../../../functions/src/services/message/test_update_message_usage.md)
- [test_update_message_metrics_with_perf_data](../../../../../functions/src/services/message/test_update_message_metrics_with_perf_data.md)
- [test_update_message_metrics_without_perf_data](../../../../../functions/src/services/message/test_update_message_metrics_without_perf_data.md)