---
type: Rust Method
title: file_only
resource: src/llm/tools/save_memory.rs#L44-L46
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/tools/save_memory/execute_creates_memory_file_with_header_and_fact
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/save_memory/execute_appends_to_existing_memory_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/save_memory/execute_does_not_duplicate_an_identical_fact
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/save_memory/memory_persists_across_different_sessions_in_the_same_directory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/save_memory/execute_blocked_in_read_only_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/save_memory/validate_input_rejects_empty_fact
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/save_memory/only_global_saves_with_a_store_need_approval
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn file_only() -> Self`

# Called by

- [execute_creates_memory_file_with_header_and_fact](../../../../../../functions/src/llm/tools/save_memory/execute_creates_memory_file_with_header_and_fact.md)
- [execute_appends_to_existing_memory_file](../../../../../../functions/src/llm/tools/save_memory/execute_appends_to_existing_memory_file.md)
- [execute_does_not_duplicate_an_identical_fact](../../../../../../functions/src/llm/tools/save_memory/execute_does_not_duplicate_an_identical_fact.md)
- [memory_persists_across_different_sessions_in_the_same_directory](../../../../../../functions/src/llm/tools/save_memory/memory_persists_across_different_sessions_in_the_same_directory.md)
- [execute_blocked_in_read_only_mode](../../../../../../functions/src/llm/tools/save_memory/execute_blocked_in_read_only_mode.md)
- [validate_input_rejects_empty_fact](../../../../../../functions/src/llm/tools/save_memory/validate_input_rejects_empty_fact.md)
- [only_global_saves_with_a_store_need_approval](../../../../../../functions/src/llm/tools/save_memory/only_global_saves_with_a_store_need_approval.md)