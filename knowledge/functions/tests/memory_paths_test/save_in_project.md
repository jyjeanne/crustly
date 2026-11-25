---
type: Rust Function
title: save_in_project
resource: tests/memory_paths_test.rs#L25-L43
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/save
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/tests/memory_paths_test/paths_must_be_relative_and_few
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_paths_test/a_mentioned_path_brings_its_memory_in_even_under_a_tight_budget
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_paths_test/a_path_memory_is_shown_once_per_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_paths_test/tool_paths_match_when_absolute_or_relative_to_a_subdirectory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_paths_test/an_absolute_path_in_the_message_brings_its_memory_in
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn save_in_project(store: &MemoryStore, project: &ProjectIdentity, content: &str) -> Uuid`

# Calls

- [save](../../../functions/src/memory/MemoryStore/save.md)

# Called by

- [paths_must_be_relative_and_few](../../../functions/tests/memory_paths_test/paths_must_be_relative_and_few.md)
- [a_mentioned_path_brings_its_memory_in_even_under_a_tight_budget](../../../functions/tests/memory_paths_test/a_mentioned_path_brings_its_memory_in_even_under_a_tight_budget.md)
- [a_path_memory_is_shown_once_per_session](../../../functions/tests/memory_paths_test/a_path_memory_is_shown_once_per_session.md)
- [tool_paths_match_when_absolute_or_relative_to_a_subdirectory](../../../functions/tests/memory_paths_test/tool_paths_match_when_absolute_or_relative_to_a_subdirectory.md)
- [an_absolute_path_in_the_message_brings_its_memory_in](../../../functions/tests/memory_paths_test/an_absolute_path_in_the_message_brings_its_memory_in.md)