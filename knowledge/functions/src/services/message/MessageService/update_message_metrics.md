---
type: Rust Method
title: update_message_metrics
resource: src/services/message.rs#L133-L157
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/services/message/MessageService/get_message_required
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/message/MessageRepository/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceContext/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/message/MessageRepository/update
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/llm/agent/service/AgentService/send_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
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

`pub async fn update_message_metrics( &self, id: Uuid, provider_name: &str, perf_metrics: Option<&crate::llm::provider::PerfMetrics>, ) -> Result<()>`

# Calls

- [get_message_required](../../../../../functions/src/services/message/MessageService/get_message_required.md)
- [context](../../../../../functions/src/services/ServiceManager/context.md)
- [new](../../../../../functions/src/db/repository/message/MessageRepository/new.md)
- [pool](../../../../../functions/src/services/ServiceContext/pool.md)
- [update](../../../../../functions/src/db/repository/message/MessageRepository/update.md)

# Called by

- [send_message](../../../../../functions/src/llm/agent/service/AgentService/send_message.md)
- [send_message_with_tools_inner](../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [test_update_message_metrics_with_perf_data](../../../../../functions/src/services/message/test_update_message_metrics_with_perf_data.md)
- [test_update_message_metrics_without_perf_data](../../../../../functions/src/services/message/test_update_message_metrics_without_perf_data.md)