---
type: Rust Function
title: request
resource: tests/memory_store_test.rs#L44-L58
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/tests/memory_store_test/recall_includes_own_project_and_global_only
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_store_test/recent_work_is_added_only_on_the_first_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_store_test/recall_records_what_it_sent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_store_test/recall_is_none_when_nothing_is_stored
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn request<'a>( project: &'a ProjectIdentity, message: &'a str, root: &'a Path, first: bool, ) -> RecallRequest<'a>`

# Called by

- [recall_includes_own_project_and_global_only](../../../functions/tests/memory_store_test/recall_includes_own_project_and_global_only.md)
- [recent_work_is_added_only_on_the_first_request](../../../functions/tests/memory_store_test/recent_work_is_added_only_on_the_first_request.md)
- [recall_records_what_it_sent](../../../functions/tests/memory_store_test/recall_records_what_it_sent.md)
- [recall_is_none_when_nothing_is_stored](../../../functions/tests/memory_store_test/recall_is_none_when_nothing_is_stored.md)