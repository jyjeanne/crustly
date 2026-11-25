---
type: Rust Method
title: list_all
resource: src/db/repository/memory.rs#L539-L546
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/memory/MemoryStore/all
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_review_fixes_test/the_database_listing_covers_every_project
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn list_all(&self) -> Result<Vec<Memory>>`

# Called by

- [all](../../../../../../functions/src/memory/MemoryStore/all.md)
- [the_database_listing_covers_every_project](../../../../../../functions/tests/memory_review_fixes_test/the_database_listing_covers_every_project.md)