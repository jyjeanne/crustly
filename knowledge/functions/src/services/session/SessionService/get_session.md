---
type: Rust Method
title: get_session
resource: src/services/session.rs#L51-L54
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/repository/session/SessionRepository/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceContext/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/session/SessionRepository/find_by_id
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/prepare_message_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/SessionService/get_session_required
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/test_get_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/test_delete_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/load_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/error_scenarios_test/test_error_database_concurrent_access
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/error_scenarios_test/test_error_recovery_after_failure
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/integration_test/test_end_to_end_cost_tracking
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/integration_test/test_end_to_end_token_usage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/integration_test/test_database_persistence
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn get_session(&self, id: Uuid) -> Result<Option<Session>>`

# Calls

- [new](../../../../../functions/src/db/repository/session/SessionRepository/new.md)
- [pool](../../../../../functions/src/services/ServiceContext/pool.md)
- [find_by_id](../../../../../functions/src/db/repository/session/SessionRepository/find_by_id.md)
- [context](../../../../../functions/src/services/ServiceManager/context.md)

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