---
type: Rust Method
title: execute
resource: src/llm/tools/write.rs#L87-L222
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/trait/ToolResult/error
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/sandbox/check_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/error/validate_path_safety
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/file_read_cache/FileReadCache/check
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/file_read_cache/FileFingerprint/of
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/write
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/file_read_cache/FileReadCache/record
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/ToolResult/success
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/trait/ToolResult/with_metadata
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/SkillSource/display
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/write/test_write_file
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/write/test_write_file_with_create_dirs
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/write/test_write_file_missing_parent_dir
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/write/test_write_file_accepts_file_path_alias
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/write/test_overwrite_existing_file
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/write/test_overwrite_rejects_a_file_never_read_this_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/write/test_overwrite_rejects_a_file_changed_since_it_was_read
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/write/test_creating_a_new_file_needs_no_prior_read
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/write/test_write_then_overwrite_does_not_require_a_re_read
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [error](../../../../../../../functions/src/llm/tools/trait/ToolResult/error.md)
- [check_path](../../../../../../../functions/src/llm/tools/sandbox/check_path.md)
- [validate_path_safety](../../../../../../../functions/src/llm/tools/error/validate_path_safety.md)
- [contains](../../../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
- [check](../../../../../../../functions/src/llm/tools/file_read_cache/FileReadCache/check.md)
- [of](../../../../../../../functions/src/llm/tools/file_read_cache/FileFingerprint/of.md)
- [write](../../../../../../../functions/src/memory/project/write.md)
- [record](../../../../../../../functions/src/llm/tools/file_read_cache/FileReadCache/record.md)
- [success](../../../../../../../functions/src/llm/tools/trait/ToolResult/success.md)
- [with_metadata](../../../../../../../functions/src/llm/tools/trait/ToolResult/with_metadata.md)
- [display](../../../../../../../functions/src/llm/tools/skill/SkillSource/display.md)

# Called by

- [test_write_file](../../../../../../../functions/src/llm/tools/write/test_write_file.md)
- [test_write_file_with_create_dirs](../../../../../../../functions/src/llm/tools/write/test_write_file_with_create_dirs.md)
- [test_write_file_missing_parent_dir](../../../../../../../functions/src/llm/tools/write/test_write_file_missing_parent_dir.md)
- [test_write_file_accepts_file_path_alias](../../../../../../../functions/src/llm/tools/write/test_write_file_accepts_file_path_alias.md)
- [test_overwrite_existing_file](../../../../../../../functions/src/llm/tools/write/test_overwrite_existing_file.md)
- [test_overwrite_rejects_a_file_never_read_this_session](../../../../../../../functions/src/llm/tools/write/test_overwrite_rejects_a_file_never_read_this_session.md)
- [test_overwrite_rejects_a_file_changed_since_it_was_read](../../../../../../../functions/src/llm/tools/write/test_overwrite_rejects_a_file_changed_since_it_was_read.md)
- [test_creating_a_new_file_needs_no_prior_read](../../../../../../../functions/src/llm/tools/write/test_creating_a_new_file_needs_no_prior_read.md)
- [test_write_then_overwrite_does_not_require_a_re_read](../../../../../../../functions/src/llm/tools/write/test_write_then_overwrite_does_not_require_a_re_read.md)