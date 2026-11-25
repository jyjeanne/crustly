---
type: Rust Method
title: semantic_scores
resource: src/memory/mod.rs#L376-L429
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/semantic/current_index
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/warn_query_once
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/semantic/normalise
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/MemoryStore/try_recall
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn semantic_scores( &self, project: &ProjectId, message: &str, now: DateTime<Utc>, ) -> Option<semantic::SemanticHits>`

# Calls

- [current_index](../../../../functions/src/memory/semantic/current_index.md)
- [warn_query_once](../../../../functions/src/memory/MemoryStore/warn_query_once.md)
- [normalise](../../../../functions/src/memory/semantic/normalise.md)

# Called by

- [try_recall](../../../../functions/src/memory/MemoryStore/try_recall.md)