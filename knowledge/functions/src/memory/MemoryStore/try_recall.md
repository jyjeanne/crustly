---
type: Rust Method
title: try_recall
resource: src/memory/mod.rs#L441-L576
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/MemoryRepository/candidates
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/recall/fts_match_expr
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/fts_search
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/semantic_scores
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/paths/mentioned_paths
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/paths/project_relative
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/path_globs_for
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/paths/matches
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/anchors/MemoryStore/anchor_hits
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/recent_work
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/context/token_count
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/recall/select
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/semantic/drop_near_duplicates
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/record_recall
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/anchors/MemoryStore/anchor_locations
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/recall/render_block_located
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/MemoryStore/recall
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn try_recall(&self, request: &RecallRequest<'_>) -> Result<Option<String>>`

# Calls

- [candidates](../../../../functions/src/db/repository/memory/MemoryRepository/candidates.md)
- [fts_match_expr](../../../../functions/src/memory/recall/fts_match_expr.md)
- [fts_search](../../../../functions/src/db/repository/memory/MemoryRepository/fts_search.md)
- [semantic_scores](../../../../functions/src/memory/MemoryStore/semantic_scores.md)
- [mentioned_paths](../../../../functions/src/memory/paths/mentioned_paths.md)
- [project_relative](../../../../functions/src/memory/paths/project_relative.md)
- [path_globs_for](../../../../functions/src/db/repository/memory/MemoryRepository/path_globs_for.md)
- [matches](../../../../functions/src/memory/paths/matches.md)
- [anchor_hits](../../../../functions/src/memory/anchors/MemoryStore/anchor_hits.md)
- [find](../../../../functions/src/memory/MemoryStore/find.md)
- [recent_work](../../../../functions/src/memory/MemoryStore/recent_work.md)
- [token_count](../../../../functions/src/llm/agent/context/token_count.md)
- [contains](../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
- [select](../../../../functions/src/memory/recall/select.md)
- [drop_near_duplicates](../../../../functions/src/memory/semantic/drop_near_duplicates.md)
- [record_recall](../../../../functions/src/db/repository/memory/MemoryRepository/record_recall.md)
- [anchor_locations](../../../../functions/src/memory/anchors/MemoryStore/anchor_locations.md)
- [render_block_located](../../../../functions/src/memory/recall/render_block_located.md)

# Called by

- [recall](../../../../functions/src/memory/MemoryStore/recall.md)