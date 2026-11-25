---
type: Rust Method
title: send_message_with_tools_cancellable
resource: src/llm/agent/service.rs#L1238-L1255
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/cancelling_during_an_approval_prompt_ends_the_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/subagents/execute
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) async fn send_message_with_tools_cancellable( &self, session_id: Uuid, user_message: String, model: Option<String>, read_only_mode: bool, cancel: CancellationToken, ) -> Result<AgentResponse>`

# Calls

- [send_message_with_tools_inner](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)

# Called by

- [cancelling_during_an_approval_prompt_ends_the_request](../../../../../../functions/src/llm/agent/service/cancelling_during_an_approval_prompt_ends_the_request.md)
- [execute](../../../../../../functions/src/llm/agent/service/subagents/execute.md)