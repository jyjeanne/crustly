---
type: Rust Module
title: memory_migration_test
resource: tests/memory_migration_test.rs#L1-L139
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/crustly-db-database
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-borrow-cow
    resolved_by: tree-sitter
    confidence: exact
  - target: external/sha2-digest-sha384
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [a_database_from_before_memory_upgrades_and_keeps_its_sessions](../../functions/tests/memory_migration_test/a_database_from_before_memory_upgrades_and_keeps_its_sessions.md)
- [a_checksum_recorded_with_other_line_endings_is_accepted_and_updated](../../functions/tests/memory_migration_test/a_checksum_recorded_with_other_line_endings_is_accepted_and_updated.md)
- [an_unrelated_checksum_mismatch_is_still_refused](../../functions/tests/memory_migration_test/an_unrelated_checksum_mismatch_is_still_refused.md)

# Imports

- `crustly::db::Database`
- `std::borrow::Cow`
- `sha2::{Digest, Sha384}`

# Member of

- [crustly](../../packages/crustly.md)