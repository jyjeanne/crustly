---
type: Rust Method
title: recent_work
resource: src/memory/mod.rs#L583-L626
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/MemoryRepository/episodes_for
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/context/token_count
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/MemoryStore/try_recall
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn recent_work( &self, request: &RecallRequest<'_>, project_id: &ProjectId, budget: u32, ) -> (Vec<String>, Vec<Uuid>)`

# Calls

- [episodes_for](../../../../functions/src/db/repository/memory/MemoryRepository/episodes_for.md)
- [token_count](../../../../functions/src/llm/agent/context/token_count.md)

# Called by

- [try_recall](../../../../functions/src/memory/MemoryStore/try_recall.md)