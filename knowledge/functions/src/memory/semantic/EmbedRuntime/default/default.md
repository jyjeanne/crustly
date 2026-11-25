---
type: Rust Method
title: default
resource: src/memory/semantic.rs#L43-L54
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/memory/embedder_from_config
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_benchmark_test/the_harness_runs_and_meets_the_ci_floor
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_benchmark_test/real_embedder_benchmark
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_consolidate_test/near_duplicates_in_the_merge_band_are_merged_on_apply
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_consolidate_test/the_merge_keeps_categories_apart
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_consolidate_test/the_merge_keeps_the_users_copy_over_a_newer_extracted_one
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn default() -> Self`

# Called by

- [embedder_from_config](../../../../../../functions/src/memory/embedder_from_config.md)
- [the_harness_runs_and_meets_the_ci_floor](../../../../../../functions/tests/memory_benchmark_test/the_harness_runs_and_meets_the_ci_floor.md)
- [real_embedder_benchmark](../../../../../../functions/tests/memory_benchmark_test/real_embedder_benchmark.md)
- [near_duplicates_in_the_merge_band_are_merged_on_apply](../../../../../../functions/tests/memory_consolidate_test/near_duplicates_in_the_merge_band_are_merged_on_apply.md)
- [the_merge_keeps_categories_apart](../../../../../../functions/tests/memory_consolidate_test/the_merge_keeps_categories_apart.md)
- [the_merge_keeps_the_users_copy_over_a_newer_extracted_one](../../../../../../functions/tests/memory_consolidate_test/the_merge_keeps_the_users_copy_over_a_newer_extracted_one.md)