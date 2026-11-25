---
type: Rust Method
title: add_sources
resource: src/db/repository/memory.rs#L358-L369
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/memory/consolidate/MemoryStore/lesson_from_episodes
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn add_sources(&self, consolidated: Uuid, sources: &[Uuid]) -> Result<()>`

# Called by

- [lesson_from_episodes](../../../../../../functions/src/memory/consolidate/MemoryStore/lesson_from_episodes.md)