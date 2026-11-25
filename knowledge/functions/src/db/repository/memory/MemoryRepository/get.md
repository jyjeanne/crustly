---
type: Rust Method
title: get
resource: src/db/repository/memory.rs#L471-L479
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/memory/anchors/MemoryStore/anchor_automatically
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/anchors/MemoryStore/set_anchors
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/anchors/MemoryStore/drifted
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/extraction/MemoryStore/approve
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/MemoryStore/save_with
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/MemoryStore/get
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/MemoryStore/edit
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/MemoryStore/promote
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/MemoryStore/add_paths
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/MemoryStore/memories_for_path
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/semantic/spawn_worker
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/semantic/archive_near_duplicates
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_review_fixes_test/forget_is_refused_in_plan_mode_and_deletes_nothing
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_store_test/user_repeats_raise_importance_and_model_repeats_do_not
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn get(&self, id: Uuid) -> Result<Option<Memory>>`

# Called by

- [anchor_automatically](../../../../../../functions/src/memory/anchors/MemoryStore/anchor_automatically.md)
- [set_anchors](../../../../../../functions/src/memory/anchors/MemoryStore/set_anchors.md)
- [drifted](../../../../../../functions/src/memory/anchors/MemoryStore/drifted.md)
- [approve](../../../../../../functions/src/memory/extraction/MemoryStore/approve.md)
- [save_with](../../../../../../functions/src/memory/MemoryStore/save_with.md)
- [get](../../../../../../functions/src/memory/MemoryStore/get.md)
- [edit](../../../../../../functions/src/memory/MemoryStore/edit.md)
- [promote](../../../../../../functions/src/memory/MemoryStore/promote.md)
- [add_paths](../../../../../../functions/src/memory/MemoryStore/add_paths.md)
- [memories_for_path](../../../../../../functions/src/memory/MemoryStore/memories_for_path.md)
- [spawn_worker](../../../../../../functions/src/memory/semantic/spawn_worker.md)
- [archive_near_duplicates](../../../../../../functions/src/memory/semantic/archive_near_duplicates.md)
- [forget_is_refused_in_plan_mode_and_deletes_nothing](../../../../../../functions/tests/memory_review_fixes_test/forget_is_refused_in_plan_mode_and_deletes_nothing.md)
- [user_repeats_raise_importance_and_model_repeats_do_not](../../../../../../functions/tests/memory_store_test/user_repeats_raise_importance_and_model_repeats_do_not.md)