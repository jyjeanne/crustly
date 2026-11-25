---
type: Rust Method
title: remove
resource: src/memory/vector_index.rs#L75-L77
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/tools/task/TaskTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/extraction/MemoryStore/trigger
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/extraction/MemoryStore/finish_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/semantic_scores
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/insert_replaces_and_remove_deletes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/forget_asks_first_and_deletes_only_with_yes
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn remove(&mut self, id: &Uuid)`

# Called by

- [execute](../../../../../functions/src/llm/tools/task/TaskTool/tool/execute.md)
- [trigger](../../../../../functions/src/memory/extraction/MemoryStore/trigger.md)
- [finish_session](../../../../../functions/src/memory/extraction/MemoryStore/finish_session.md)
- [semantic_scores](../../../../../functions/src/memory/MemoryStore/semantic_scores.md)
- [insert_replaces_and_remove_deletes](../../../../../functions/src/memory/vector_index/insert_replaces_and_remove_deletes.md)
- [forget_asks_first_and_deletes_only_with_yes](../../../../../functions/src/tui/app/forget_asks_first_and_deletes_only_with_yes.md)