---
type: Rust Method
title: events
resource: src/memory/mod.rs#L689-L691
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/MemoryRepository/events
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn events(&self, id: Uuid) -> Result<Vec<(String, String, chrono::DateTime<Utc>)>>`

# Calls

- [events](../../../../functions/src/db/repository/memory/MemoryRepository/events.md)

# Called by

- [cmd_memory](../../../../functions/src/cli/cmd_memory.md)