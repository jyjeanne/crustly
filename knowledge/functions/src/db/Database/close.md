---
type: Rust Method
title: close
resource: src/db/mod.rs#L187-L191
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/tests/memory_migration_test/a_database_from_before_memory_upgrades_and_keeps_its_sessions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_migration_test/a_checksum_recorded_with_other_line_endings_is_accepted_and_updated
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_migration_test/an_unrelated_checksum_mismatch_is_still_refused
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn close(self) -> Result<()>`

# Called by

- [a_database_from_before_memory_upgrades_and_keeps_its_sessions](../../../../functions/tests/memory_migration_test/a_database_from_before_memory_upgrades_and_keeps_its_sessions.md)
- [a_checksum_recorded_with_other_line_endings_is_accepted_and_updated](../../../../functions/tests/memory_migration_test/a_checksum_recorded_with_other_line_endings_is_accepted_and_updated.md)
- [an_unrelated_checksum_mismatch_is_still_refused](../../../../functions/tests/memory_migration_test/an_unrelated_checksum_mismatch_is_still_refused.md)