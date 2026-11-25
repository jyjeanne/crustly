---
type: Rust Method
title: of
resource: src/llm/tools/file_read_cache.rs#L35-L40
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/apply_patch/seeded_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/apply_patch/execute_is_atomic_across_files_on_failure
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/edit/EditTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/edit/seeded_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/read/ReadTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/write/WriteTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/write/test_overwrite_existing_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/write/test_overwrite_rejects_a_file_changed_since_it_was_read
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn of(metadata: &std::fs::Metadata) -> Self`

# Called by

- [execute](../../../../../../functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/execute.md)
- [seeded_context](../../../../../../functions/src/llm/tools/apply_patch/seeded_context.md)
- [execute_is_atomic_across_files_on_failure](../../../../../../functions/src/llm/tools/apply_patch/execute_is_atomic_across_files_on_failure.md)
- [execute](../../../../../../functions/src/llm/tools/edit/EditTool/tool/execute.md)
- [seeded_context](../../../../../../functions/src/llm/tools/edit/seeded_context.md)
- [execute](../../../../../../functions/src/llm/tools/read/ReadTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/write/WriteTool/tool/execute.md)
- [test_overwrite_existing_file](../../../../../../functions/src/llm/tools/write/test_overwrite_existing_file.md)
- [test_overwrite_rejects_a_file_changed_since_it_was_read](../../../../../../functions/src/llm/tools/write/test_overwrite_rejects_a_file_changed_since_it_was_read.md)