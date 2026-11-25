---
type: Rust Function
title: row_to_memory
resource: src/db/repository/memory.rs#L830-L853
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/models/PlanTaskStatus/parse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/timestamp
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/db/repository/memory/MemoryRepository/fts_search
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn row_to_memory(row: &sqlx::sqlite::SqliteRow) -> Result<Memory>`

# Calls

- [parse](../../../../../functions/src/db/models/PlanTaskStatus/parse.md)
- [timestamp](../../../../../functions/src/db/repository/memory/timestamp.md)

# Called by

- [fts_search](../../../../../functions/src/db/repository/memory/MemoryRepository/fts_search.md)