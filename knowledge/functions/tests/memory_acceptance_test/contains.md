---
type: Rust Function
title: contains
resource: tests/memory_acceptance_test.rs#L281-L285
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/tests/memory_acceptance_test/a_secret_never_reaches_the_database_file_or_its_wal
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn contains(haystack: &[u8], needle: &[u8]) -> bool`

# Called by

- [a_secret_never_reaches_the_database_file_or_its_wal](../../../functions/tests/memory_acceptance_test/a_secret_never_reaches_the_database_file_or_its_wal.md)