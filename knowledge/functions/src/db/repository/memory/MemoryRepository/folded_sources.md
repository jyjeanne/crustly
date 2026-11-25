---
type: Rust Method
title: folded_sources
resource: src/db/repository/memory.rs#L343-L355
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/models/PlanTaskStatus/parse
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/consolidate/MemoryStore/lesson_from_episodes
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn folded_sources( &self, project_id: &ProjectId, ) -> Result<std::collections::HashSet<Uuid>>`

# Calls

- [parse](../../../../../../functions/src/db/models/PlanTaskStatus/parse.md)

# Called by

- [lesson_from_episodes](../../../../../../functions/src/memory/consolidate/MemoryStore/lesson_from_episodes.md)