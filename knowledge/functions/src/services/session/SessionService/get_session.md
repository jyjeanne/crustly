---
type: Rust Method
title: get_session
resource: src/services/session.rs#L53-L56
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
  - functions/src/llm/agent/service/AgentService/prepare_message_context
  - functions/src/services/session/SessionService/get_session_required
  - functions/src/services/session/test_get_session
  - functions/src/services/session/test_delete_session
  - functions/src/tui/app/App/load_session
  - functions/tests/error_scenarios_test/test_error_database_concurrent_access
  - functions/tests/error_scenarios_test/test_error_recovery_after_failure
  - functions/tests/integration_test/test_end_to_end_cost_tracking
  - functions/tests/integration_test/test_end_to_end_token_usage
  - functions/tests/integration_test/test_database_persistence
---

# Signature

`pub async fn get_session(&self, id: Uuid) -> Result<Option<Session>>`

# Called by

- [send_message_with_tools_inner](../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [prepare_message_context](../../../../../functions/src/llm/agent/service/AgentService/prepare_message_context.md)
- [get_session_required](../../../../../functions/src/services/session/SessionService/get_session_required.md)
- [test_get_session](../../../../../functions/src/services/session/test_get_session.md)
- [test_delete_session](../../../../../functions/src/services/session/test_delete_session.md)
- [load_session](../../../../../functions/src/tui/app/App/load_session.md)
- [test_error_database_concurrent_access](../../../../../functions/tests/error_scenarios_test/test_error_database_concurrent_access.md)
- [test_error_recovery_after_failure](../../../../../functions/tests/error_scenarios_test/test_error_recovery_after_failure.md)
- [test_end_to_end_cost_tracking](../../../../../functions/tests/integration_test/test_end_to_end_cost_tracking.md)
- [test_end_to_end_token_usage](../../../../../functions/tests/integration_test/test_end_to_end_token_usage.md)
- [test_database_persistence](../../../../../functions/tests/integration_test/test_database_persistence.md)