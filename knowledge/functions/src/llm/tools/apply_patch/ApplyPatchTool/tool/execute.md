---
type: Rust Method
title: execute
resource: src/llm/tools/apply_patch.rs#L343-L518
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/trait/ToolResult/error
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/apply_patch/parse_patch
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/sandbox/check_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/error/validate_path_safety
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/exists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/error/validate_file_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/file_read_cache/FileReadCache/check
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/file_read_cache/FileFingerprint/of
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/apply_patch/apply_hunks
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
  - target: functions/src/llm/tools/skill/SkillSource/display
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/apply_patch/execute_updates_an_existing_file
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/apply_patch/execute_adds_a_new_file
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/apply_patch/execute_add_file_that_already_exists_fails
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/apply_patch/execute_deletes_a_file
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/apply_patch/execute_renames_via_move_to
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/apply_patch/execute_applies_multiple_file_ops_in_one_patch
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/apply_patch/execute_is_atomic_across_files_on_failure
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/apply_patch/execute_blocked_in_read_only_mode
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/apply_patch/execute_update_rejects_a_file_never_read_this_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/apply_patch/execute_add_and_delete_need_no_prior_read
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [error](../../../../../../../functions/src/llm/tools/trait/ToolResult/error.md)
- [parse_patch](../../../../../../../functions/src/llm/tools/apply_patch/parse_patch.md)
- [check_path](../../../../../../../functions/src/llm/tools/sandbox/check_path.md)
- [validate_path_safety](../../../../../../../functions/src/llm/tools/error/validate_path_safety.md)
- [exists](../../../../../../../functions/src/memory/code_knowledge/CodeKnowledge/exists.md)
- [validate_file_path](../../../../../../../functions/src/llm/tools/error/validate_file_path.md)
- [check](../../../../../../../functions/src/llm/tools/file_read_cache/FileReadCache/check.md)
- [of](../../../../../../../functions/src/llm/tools/file_read_cache/FileFingerprint/of.md)
- [apply_hunks](../../../../../../../functions/src/llm/tools/apply_patch/apply_hunks.md)
- [write](../../../../../../../functions/src/memory/project/write.md)
- [record](../../../../../../../functions/src/llm/tools/file_read_cache/FileReadCache/record.md)
- [success](../../../../../../../functions/src/llm/tools/trait/ToolResult/success.md)
- [display](../../../../../../../functions/src/llm/tools/skill/SkillSource/display.md)

# Called by

- [execute_updates_an_existing_file](../../../../../../../functions/src/llm/tools/apply_patch/execute_updates_an_existing_file.md)
- [execute_adds_a_new_file](../../../../../../../functions/src/llm/tools/apply_patch/execute_adds_a_new_file.md)
- [execute_add_file_that_already_exists_fails](../../../../../../../functions/src/llm/tools/apply_patch/execute_add_file_that_already_exists_fails.md)
- [execute_deletes_a_file](../../../../../../../functions/src/llm/tools/apply_patch/execute_deletes_a_file.md)
- [execute_renames_via_move_to](../../../../../../../functions/src/llm/tools/apply_patch/execute_renames_via_move_to.md)
- [execute_applies_multiple_file_ops_in_one_patch](../../../../../../../functions/src/llm/tools/apply_patch/execute_applies_multiple_file_ops_in_one_patch.md)
- [execute_is_atomic_across_files_on_failure](../../../../../../../functions/src/llm/tools/apply_patch/execute_is_atomic_across_files_on_failure.md)
- [execute_blocked_in_read_only_mode](../../../../../../../functions/src/llm/tools/apply_patch/execute_blocked_in_read_only_mode.md)
- [execute_update_rejects_a_file_never_read_this_session](../../../../../../../functions/src/llm/tools/apply_patch/execute_update_rejects_a_file_never_read_this_session.md)
- [execute_add_and_delete_need_no_prior_read](../../../../../../../functions/src/llm/tools/apply_patch/execute_add_and_delete_need_no_prior_read.md)