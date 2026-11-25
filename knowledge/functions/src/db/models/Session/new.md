---
type: Rust Method
title: new
resource: src/db/models.rs#L223-L236
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/benches/database/bench_session_create
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/benches/database/bench_session_get
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/benches/database/bench_session_list
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/benches/database/bench_message_insert
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/benches/database/bench_message_query
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/models/test_session_new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/models/test_session_archived
    resolved_by: tree-sitter-receiver
    confidence: semantic
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
  - target: functions/src/db/repository/plan/test_multiple_sessions_multiple_plans
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
  - target: functions/tests/plan_mode_integration_test/setup_test_env
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/plan_mode_integration_test/test_multiple_sessions_with_separate_plans
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(title: Option<String>, model: Option<String>) -> Self`

# Called by

- [bench_session_create](../../../../../functions/benches/database/bench_session_create.md)
- [bench_session_get](../../../../../functions/benches/database/bench_session_get.md)
- [bench_session_list](../../../../../functions/benches/database/bench_session_list.md)
- [bench_message_insert](../../../../../functions/benches/database/bench_message_insert.md)
- [bench_message_query](../../../../../functions/benches/database/bench_message_query.md)
- [test_session_new](../../../../../functions/src/db/models/test_session_new.md)
- [test_session_archived](../../../../../functions/src/db/models/test_session_archived.md)
- [test_file_crud](../../../../../functions/src/db/repository/file/test_file_crud.md)
- [test_file_list_by_session](../../../../../functions/src/db/repository/file/test_file_list_by_session.md)
- [test_message_crud](../../../../../functions/src/db/repository/message/test_message_crud.md)
- [test_message_list_by_session](../../../../../functions/src/db/repository/message/test_message_list_by_session.md)
- [setup_test_db](../../../../../functions/src/db/repository/plan/setup_test_db.md)
- [test_multiple_sessions_multiple_plans](../../../../../functions/src/db/repository/plan/test_multiple_sessions_multiple_plans.md)
- [test_session_crud](../../../../../functions/src/db/repository/session/test_session_crud.md)
- [test_session_archive](../../../../../functions/src/db/repository/session/test_session_archive.md)
- [setup_test_service](../../../../../functions/src/services/plan/setup_test_service.md)
- [setup_test_env](../../../../../functions/tests/plan_mode_integration_test/setup_test_env.md)
- [test_multiple_sessions_with_separate_plans](../../../../../functions/tests/plan_mode_integration_test/test_multiple_sessions_with_separate_plans.md)