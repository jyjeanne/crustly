---
type: Rust Function
title: setup_test_service
resource: src/services/plan.rs#L339-L362
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/db/Database/run_migrations
  called_by:
  - functions/src/services/plan/test_service_create_and_find
  - functions/src/services/plan/test_service_update
  - functions/src/services/plan/test_service_delete
  - functions/src/services/plan/test_service_find_by_session_id
  - functions/src/services/plan/test_service_get_most_recent_plan
  - functions/src/services/plan/test_service_export_to_json
  - functions/src/services/plan/test_service_import_from_json
  - functions/src/services/plan/test_service_export_import_roundtrip
  - functions/src/services/plan/test_service_atomic_json_write
  - functions/src/services/plan/test_service_json_import_nonexistent_file
  - functions/src/services/plan/test_service_json_import_invalid_json
---

# Signature

`async fn setup_test_service() -> (Database, PlanService, Session, TempDir)`

# Calls

- [run_migrations](../../../../functions/src/db/Database/run_migrations.md)

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