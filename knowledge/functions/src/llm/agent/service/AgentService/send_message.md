---
type: Rust Method
title: send_message
resource: src/llm/agent/service.rs#L1105-L1179
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/AgentService/prepare_message_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/record_calibration
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/final_text_and_thinking
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/MessageService/create_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/MessageService/update_message_usage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/MessageService/update_message_metrics
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/SessionService/update_session_usage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/usage
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/estimated_context_overflow_is_rejected_without_tools
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/error_scenarios_test/test_error_empty_message
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/integration_test/test_end_to_end_system_prompt
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn send_message( &self, session_id: Uuid, user_message: String, model: Option<String>, ) -> Result<AgentResponse>`

# Calls

- [prepare_message_context](../../../../../../functions/src/llm/agent/service/AgentService/prepare_message_context.md)
- [record_calibration](../../../../../../functions/src/llm/agent/service/AgentService/record_calibration.md)
- [final_text_and_thinking](../../../../../../functions/src/llm/agent/service/AgentService/final_text_and_thinking.md)
- [create_message](../../../../../../functions/src/services/message/MessageService/create_message.md)
- [update_message_usage](../../../../../../functions/src/services/message/MessageService/update_message_usage.md)
- [update_message_metrics](../../../../../../functions/src/services/message/MessageService/update_message_metrics.md)
- [update_session_usage](../../../../../../functions/src/services/session/SessionService/update_session_usage.md)
- [usage](../../../../../../functions/src/llm/agent/budget/ContextBudget/usage.md)

# Called by

- [estimated_context_overflow_is_rejected_without_tools](../../../../../../functions/src/llm/agent/service/estimated_context_overflow_is_rejected_without_tools.md)
- [test_error_empty_message](../../../../../../functions/tests/error_scenarios_test/test_error_empty_message.md)
- [test_end_to_end_system_prompt](../../../../../../functions/tests/integration_test/test_end_to_end_system_prompt.md)