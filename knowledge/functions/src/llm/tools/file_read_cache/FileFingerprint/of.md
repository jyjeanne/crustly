---
type: Rust Method
title: of
resource: src/llm/tools/file_read_cache.rs#L35-L40
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/len
  called_by:
  - functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/execute
  - functions/src/llm/tools/apply_patch/seeded_context
  - functions/src/llm/tools/apply_patch/execute_is_atomic_across_files_on_failure
  - functions/src/llm/tools/edit/EditTool/tool/execute
  - functions/src/llm/tools/edit/seeded_context
  - functions/src/llm/tools/read/ReadTool/tool/execute
  - functions/src/llm/tools/write/WriteTool/tool/execute
  - functions/src/llm/tools/write/test_overwrite_existing_file
  - functions/src/llm/tools/write/test_overwrite_rejects_a_file_changed_since_it_was_read
---

# Signature

`pub fn of(metadata: &std::fs::Metadata) -> Self`

# Calls

- [len](../../../../../../functions/src/config/secrets/SecretString/len.md)

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