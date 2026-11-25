---
type: Rust Method
title: load_session
resource: src/tui/app.rs#L2293-L2323
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/services/session/SessionService/get_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/MessageService/list_messages_for_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/end_session_in_background
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/provider_model
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/provider_name
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/update_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/sync_processing_state_for_current_session
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/initialize
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_sessions_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/switching_back_to_a_session_with_a_still_in_flight_request_restores_processing_state
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn load_session(&mut self, session_id: Uuid) -> Result<()>`

# Calls

- [get_session](../../../../../functions/src/services/session/SessionService/get_session.md)
- [list_messages_for_session](../../../../../functions/src/services/message/MessageService/list_messages_for_session.md)
- [end_session_in_background](../../../../../functions/src/tui/app/App/end_session_in_background.md)
- [provider_model](../../../../../functions/src/llm/agent/service/AgentService/provider_model.md)
- [provider_name](../../../../../functions/src/llm/agent/service/AgentService/provider_name.md)
- [update_session](../../../../../functions/src/services/session/SessionService/update_session.md)
- [sync_processing_state_for_current_session](../../../../../functions/src/tui/app/App/sync_processing_state_for_current_session.md)

# Called by

- [initialize](../../../../../functions/src/tui/app/App/initialize.md)
- [handle_event](../../../../../functions/src/tui/app/App/handle_event.md)
- [handle_sessions_key](../../../../../functions/src/tui/app/App/handle_sessions_key.md)
- [switching_back_to_a_session_with_a_still_in_flight_request_restores_processing_state](../../../../../functions/src/tui/app/switching_back_to_a_session_with_a_still_in_flight_request_restores_processing_state.md)