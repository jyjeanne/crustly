---
type: Rust Method
title: send_message_with_tools_and_mode_streaming
resource: src/llm/agent/service.rs#L1296-L1314
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/a_cancelled_request_stops_with_cancelled
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/send_message
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn send_message_with_tools_and_mode_streaming( &self, session_id: Uuid, user_message: String, model: Option<String>, read_only_mode: bool, chunk_tx: mpsc::UnboundedSender<String>, cancel: CancellationToken, ) -> Result<AgentResponse>`

# Calls

- [send_message_with_tools_inner](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)

# Called by

- [a_cancelled_request_stops_with_cancelled](../../../../../../functions/src/llm/agent/service/a_cancelled_request_stops_with_cancelled.md)
- [send_message](../../../../../../functions/src/tui/app/App/send_message.md)