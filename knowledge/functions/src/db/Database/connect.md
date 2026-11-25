---
type: Rust Method
title: connect
resource: src/db/mod.rs#L34-L93
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/error/ErrorInfo/with_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/benches/database/setup_test_db
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/benches/memory_recall/bench_keyword_recall
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/cli/cmd_db
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/cli/cmd_chat
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/cli/cmd_run
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/cli/reject_refuses_a_memory_that_is_not_pending
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/SqlitePool/poolext/connect_file
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/tilde_in_the_database_path_is_expanded_to_home
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/message/create_then_update_survives_a_file_backed_wal_pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/integration_test/test_database_persistence
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_acceptance_test/open
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_benchmark_test/run
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_extraction_test/database
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_migration_test/a_database_from_before_memory_upgrades_and_keeps_its_sessions
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_migration_test/a_checksum_recorded_with_other_line_endings_is_accepted_and_updated
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_migration_test/an_unrelated_checksum_mismatch_is_still_refused
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_minor_fixes_test/store
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_paths_test/store_and_project
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_paths_test/store_and_real_project
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_semantic_test/database
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn connect<P: AsRef<Path>>(path: P) -> Result<Self>`

# Calls

- [with_context](../../../../functions/src/tui/error/ErrorInfo/with_context.md)
- [context](../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [setup_test_db](../../../../functions/benches/database/setup_test_db.md)
- [bench_keyword_recall](../../../../functions/benches/memory_recall/bench_keyword_recall.md)
- [cmd_memory](../../../../functions/src/cli/cmd_memory.md)
- [cmd_db](../../../../functions/src/cli/cmd_db.md)
- [cmd_chat](../../../../functions/src/cli/cmd_chat.md)
- [cmd_run](../../../../functions/src/cli/cmd_run.md)
- [reject_refuses_a_memory_that_is_not_pending](../../../../functions/src/cli/reject_refuses_a_memory_that_is_not_pending.md)
- [connect_file](../../../../functions/src/db/SqlitePool/poolext/connect_file.md)
- [tilde_in_the_database_path_is_expanded_to_home](../../../../functions/src/db/tilde_in_the_database_path_is_expanded_to_home.md)
- [create_then_update_survives_a_file_backed_wal_pool](../../../../functions/src/services/message/create_then_update_survives_a_file_backed_wal_pool.md)
- [test_database_persistence](../../../../functions/tests/integration_test/test_database_persistence.md)
- [open](../../../../functions/tests/memory_acceptance_test/open.md)
- [run](../../../../functions/tests/memory_benchmark_test/run.md)
- [database](../../../../functions/tests/memory_extraction_test/database.md)
- [a_database_from_before_memory_upgrades_and_keeps_its_sessions](../../../../functions/tests/memory_migration_test/a_database_from_before_memory_upgrades_and_keeps_its_sessions.md)
- [a_checksum_recorded_with_other_line_endings_is_accepted_and_updated](../../../../functions/tests/memory_migration_test/a_checksum_recorded_with_other_line_endings_is_accepted_and_updated.md)
- [an_unrelated_checksum_mismatch_is_still_refused](../../../../functions/tests/memory_migration_test/an_unrelated_checksum_mismatch_is_still_refused.md)
- [store](../../../../functions/tests/memory_minor_fixes_test/store.md)
- [store_and_project](../../../../functions/tests/memory_paths_test/store_and_project.md)
- [store_and_real_project](../../../../functions/tests/memory_paths_test/store_and_real_project.md)
- [database](../../../../functions/tests/memory_semantic_test/database.md)