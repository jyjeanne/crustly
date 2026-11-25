---
type: Rust Method
title: new
resource: src/db/repository/session.rs#L30-L32
generated:
  by: okf-rs/0.8.0
relationships:
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
  - target: functions/src/services/session/SessionService/get_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/list_sessions
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/update_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/update_session_title
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/update_session_usage
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/archive_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/unarchive_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/delete_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/get_most_recent_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/count_sessions
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/count_archived_sessions
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

`pub fn new(pool: SqlitePool) -> Self`

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
- [get_session](../../../../../../functions/src/services/session/SessionService/get_session.md)
- [list_sessions](../../../../../../functions/src/services/session/SessionService/list_sessions.md)
- [update_session](../../../../../../functions/src/services/session/SessionService/update_session.md)
- [update_session_title](../../../../../../functions/src/services/session/SessionService/update_session_title.md)
- [update_session_usage](../../../../../../functions/src/services/session/SessionService/update_session_usage.md)
- [archive_session](../../../../../../functions/src/services/session/SessionService/archive_session.md)
- [unarchive_session](../../../../../../functions/src/services/session/SessionService/unarchive_session.md)
- [delete_session](../../../../../../functions/src/services/session/SessionService/delete_session.md)
- [get_most_recent_session](../../../../../../functions/src/services/session/SessionService/get_most_recent_session.md)
- [count_sessions](../../../../../../functions/src/services/session/SessionService/count_sessions.md)
- [count_archived_sessions](../../../../../../functions/src/services/session/SessionService/count_archived_sessions.md)
- [setup_test_env](../../../../../../functions/tests/plan_mode_integration_test/setup_test_env.md)
- [test_multiple_sessions_with_separate_plans](../../../../../../functions/tests/plan_mode_integration_test/test_multiple_sessions_with_separate_plans.md)