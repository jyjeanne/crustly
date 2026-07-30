---
type: Rust Method
title: update_message_metrics
resource: src/services/message.rs#L118-L142
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/services/message/MessageService/get_message_required
  called_by:
  - functions/src/llm/agent/service/AgentService/send_message
  - functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
  - functions/src/services/message/test_update_message_metrics_with_perf_data
  - functions/src/services/message/test_update_message_metrics_without_perf_data
---

# Signature

`pub async fn update_message_metrics( &self, id: Uuid, provider_name: &str, perf_metrics: Option<&crate::llm::provider::PerfMetrics>, ) -> Result<()>`

# Calls

- [get_message_required](../../../../../functions/src/services/message/MessageService/get_message_required.md)

# Called by

- [send_message](../../../../../functions/src/llm/agent/service/AgentService/send_message.md)
- [send_message_with_tools_inner](../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [test_update_message_metrics_with_perf_data](../../../../../functions/src/services/message/test_update_message_metrics_with_perf_data.md)
- [test_update_message_metrics_without_perf_data](../../../../../functions/src/services/message/test_update_message_metrics_without_perf_data.md)