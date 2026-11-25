---
type: Rust Function
title: save_request
resource: tests/memory_store_test.rs#L31-L42
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/tests/memory_store_test/saving_twice_gives_one_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_store_test/user_repeats_raise_importance_and_model_repeats_do_not
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_store_test/a_secret_is_rejected_and_nothing_is_stored
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_store_test/invalid_text_is_rejected_with_a_reason
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_store_test/a_model_cannot_replace_a_memory_the_user_saved
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_store_test/replacement_must_stay_in_the_same_scope
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_store_test/saves_are_announced_once
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_store_test/recall_includes_own_project_and_global_only
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_store_test/recent_work_is_added_only_on_the_first_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_store_test/recall_records_what_it_sent
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn save_request(scope: MemoryScope, content: &str, source: MemorySource) -> NewMemory`

# Called by

- [saving_twice_gives_one_memory](../../../functions/tests/memory_store_test/saving_twice_gives_one_memory.md)
- [user_repeats_raise_importance_and_model_repeats_do_not](../../../functions/tests/memory_store_test/user_repeats_raise_importance_and_model_repeats_do_not.md)
- [a_secret_is_rejected_and_nothing_is_stored](../../../functions/tests/memory_store_test/a_secret_is_rejected_and_nothing_is_stored.md)
- [invalid_text_is_rejected_with_a_reason](../../../functions/tests/memory_store_test/invalid_text_is_rejected_with_a_reason.md)
- [a_model_cannot_replace_a_memory_the_user_saved](../../../functions/tests/memory_store_test/a_model_cannot_replace_a_memory_the_user_saved.md)
- [replacement_must_stay_in_the_same_scope](../../../functions/tests/memory_store_test/replacement_must_stay_in_the_same_scope.md)
- [saves_are_announced_once](../../../functions/tests/memory_store_test/saves_are_announced_once.md)
- [recall_includes_own_project_and_global_only](../../../functions/tests/memory_store_test/recall_includes_own_project_and_global_only.md)
- [recent_work_is_added_only_on_the_first_request](../../../functions/tests/memory_store_test/recent_work_is_added_only_on_the_first_request.md)
- [recall_records_what_it_sent](../../../functions/tests/memory_store_test/recall_records_what_it_sent.md)