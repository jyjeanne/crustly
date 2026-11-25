---
type: Rust Method
title: prepare_message_context
resource: src/llm/agent/service.rs#L1536-L1617
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/services/session/SessionService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/get_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/MessageService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/message/MessageService/list_messages_for_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/context/AgentContext/from_db_messages
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/system_prompt_with_env
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/pdf_context/augment_message_with_pdf
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/memory_block
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/context/AgentContext/add_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/user_turn
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/RequestTools/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/measure_budget
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/is_full
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/overflow_error
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/MessageService/create_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/build_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/completion_budget
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/AgentService/send_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/send_message_streaming
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn prepare_message_context( &self, session_id: Uuid, user_message: String, model: Option<String>, ) -> Result<( String, LLMRequest, MessageService, SessionService, ContextBudget, )>`

# Calls

- [new](../../../../../../functions/src/services/session/SessionService/new.md)
- [get_session](../../../../../../functions/src/services/session/SessionService/get_session.md)
- [new](../../../../../../functions/src/services/message/MessageService/new.md)
- [list_messages_for_session](../../../../../../functions/src/services/message/MessageService/list_messages_for_session.md)
- [from_db_messages](../../../../../../functions/src/llm/agent/context/AgentContext/from_db_messages.md)
- [system_prompt_with_env](../../../../../../functions/src/llm/agent/service/AgentService/system_prompt_with_env.md)
- [augment_message_with_pdf](../../../../../../functions/src/llm/pdf_context/augment_message_with_pdf.md)
- [memory_block](../../../../../../functions/src/llm/agent/service/AgentService/memory_block.md)
- [add_message](../../../../../../functions/src/llm/agent/context/AgentContext/add_message.md)
- [user_turn](../../../../../../functions/src/llm/agent/service/user_turn.md)
- [new](../../../../../../functions/src/llm/agent/service/RequestTools/new.md)
- [measure_budget](../../../../../../functions/src/llm/agent/service/AgentService/measure_budget.md)
- [is_full](../../../../../../functions/src/llm/agent/budget/ContextBudget/is_full.md)
- [overflow_error](../../../../../../functions/src/llm/agent/budget/ContextBudget/overflow_error.md)
- [create_message](../../../../../../functions/src/services/message/MessageService/create_message.md)
- [build_request](../../../../../../functions/src/llm/agent/service/AgentService/build_request.md)
- [completion_budget](../../../../../../functions/src/llm/agent/budget/ContextBudget/completion_budget.md)

# Called by

- [send_message](../../../../../../functions/src/llm/agent/service/AgentService/send_message.md)
- [send_message_streaming](../../../../../../functions/src/llm/agent/service/AgentService/send_message_streaming.md)