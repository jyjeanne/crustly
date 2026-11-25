---
type: Rust Method
title: attach_embedder
resource: src/memory/mod.rs#L127-L149
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/semantic/queue
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/semantic/spawn_worker
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/memory/MemoryRepository/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/cli/build_memory_store
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/with_embedder
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_benchmark_test/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_consolidate_test/near_duplicates_in_the_merge_band_are_merged_on_apply
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_consolidate_test/the_merge_keeps_categories_apart
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_consolidate_test/the_merge_keeps_the_users_copy_over_a_newer_extracted_one
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn attach_embedder( mut self, embedder: Arc<dyn embed::Embedder>, runtime: semantic::EmbedRuntime, background: bool, ) -> Self`

# Calls

- [queue](../../../../functions/src/memory/semantic/queue.md)
- [spawn_worker](../../../../functions/src/memory/semantic/spawn_worker.md)
- [new](../../../../functions/src/db/repository/memory/MemoryRepository/new.md)
- [pool](../../../../functions/src/db/repository/memory/MemoryRepository/pool.md)

# Called by

- [build_memory_store](../../../../functions/src/cli/build_memory_store.md)
- [with_embedder](../../../../functions/src/memory/MemoryStore/with_embedder.md)
- [run](../../../../functions/tests/memory_benchmark_test/run.md)
- [near_duplicates_in_the_merge_band_are_merged_on_apply](../../../../functions/tests/memory_consolidate_test/near_duplicates_in_the_merge_band_are_merged_on_apply.md)
- [the_merge_keeps_categories_apart](../../../../functions/tests/memory_consolidate_test/the_merge_keeps_categories_apart.md)
- [the_merge_keeps_the_users_copy_over_a_newer_extracted_one](../../../../functions/tests/memory_consolidate_test/the_merge_keeps_the_users_copy_over_a_newer_extracted_one.md)