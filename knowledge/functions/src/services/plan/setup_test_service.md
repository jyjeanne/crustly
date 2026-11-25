---
type: Rust Function
title: setup_test_service
resource: src/services/plan.rs#L339-L362
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/Database/connect_in_memory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/run_migrations
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/ServiceContext/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/plan/PlanService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/session/SessionRepository/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/models/Session/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/session/SessionRepository/create
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/services/plan/test_service_create_and_find
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/plan/test_service_update
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/plan/test_service_delete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/plan/test_service_find_by_session_id
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/plan/test_service_get_most_recent_plan
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/plan/test_service_export_to_json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/plan/test_service_import_from_json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/plan/test_service_export_import_roundtrip
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/plan/test_service_atomic_json_write
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/plan/test_service_json_import_nonexistent_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/plan/test_service_json_import_invalid_json
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn setup_test_service() -> (Database, PlanService, Session, TempDir)`

# Calls

- [connect_in_memory](../../../../functions/src/db/Database/connect_in_memory.md)
- [run_migrations](../../../../functions/src/db/Database/run_migrations.md)
- [new](../../../../functions/src/services/ServiceContext/new.md)
- [pool](../../../../functions/src/db/Database/pool.md)
- [new](../../../../functions/src/services/plan/PlanService/new.md)
- [new](../../../../functions/src/db/repository/session/SessionRepository/new.md)
- [new](../../../../functions/src/db/models/Session/new.md)
- [create](../../../../functions/src/db/repository/session/SessionRepository/create.md)

# Called by

- [test_service_create_and_find](../../../../functions/src/services/plan/test_service_create_and_find.md)
- [test_service_update](../../../../functions/src/services/plan/test_service_update.md)
- [test_service_delete](../../../../functions/src/services/plan/test_service_delete.md)
- [test_service_find_by_session_id](../../../../functions/src/services/plan/test_service_find_by_session_id.md)
- [test_service_get_most_recent_plan](../../../../functions/src/services/plan/test_service_get_most_recent_plan.md)
- [test_service_export_to_json](../../../../functions/src/services/plan/test_service_export_to_json.md)
- [test_service_import_from_json](../../../../functions/src/services/plan/test_service_import_from_json.md)
- [test_service_export_import_roundtrip](../../../../functions/src/services/plan/test_service_export_import_roundtrip.md)
- [test_service_atomic_json_write](../../../../functions/src/services/plan/test_service_atomic_json_write.md)
- [test_service_json_import_nonexistent_file](../../../../functions/src/services/plan/test_service_json_import_nonexistent_file.md)
- [test_service_json_import_invalid_json](../../../../functions/src/services/plan/test_service_json_import_invalid_json.md)