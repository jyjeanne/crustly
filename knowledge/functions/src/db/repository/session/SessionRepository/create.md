---
type: Rust Method
title: create
resource: src/db/repository/session.rs#L46-L69
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/db/repository/file/test_file_crud
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/file/test_file_list_by_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/message/test_message_crud
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/message/test_message_list_by_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/plan/setup_test_db
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/session/test_session_crud
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/session/test_session_archive
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/plan/setup_test_service
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/create_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/plan_mode_integration_test/setup_test_env
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/plan_mode_integration_test/test_multiple_sessions_with_separate_plans
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn create(&self, session: &Session) -> Result<()>`

# Calls

- [context](../../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [test_file_crud](../../../../../../functions/src/db/repository/file/test_file_crud.md)
- [test_file_list_by_session](../../../../../../functions/src/db/repository/file/test_file_list_by_session.md)
- [test_message_crud](../../../../../../functions/src/db/repository/message/test_message_crud.md)
- [test_message_list_by_session](../../../../../../functions/src/db/repository/message/test_message_list_by_session.md)
- [setup_test_db](../../../../../../functions/src/db/repository/plan/setup_test_db.md)
- [test_session_crud](../../../../../../functions/src/db/repository/session/test_session_crud.md)
- [test_session_archive](../../../../../../functions/src/db/repository/session/test_session_archive.md)
- [setup_test_service](../../../../../../functions/src/services/plan/setup_test_service.md)
- [create_session](../../../../../../functions/src/services/session/SessionService/create_session.md)
- [setup_test_env](../../../../../../functions/tests/plan_mode_integration_test/setup_test_env.md)
- [test_multiple_sessions_with_separate_plans](../../../../../../functions/tests/plan_mode_integration_test/test_multiple_sessions_with_separate_plans.md)