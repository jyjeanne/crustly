---
type: Rust Method
title: execute
resource: src/llm/tools/edit.rs#L212-L414
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/trait/ToolResult/error
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/edit/normalize_input
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/sandbox/check_path
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
  - target: functions/src/llm/tools/edit/test_replace_with_explicit_operation_still_works
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/edit/test_qwen_code_and_claude_code_style_payload_works_with_no_operation_field
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/edit/test_replace_rejects_non_unique_match_by_default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/edit/test_replace_all_true_replaces_every_occurrence
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/edit/test_replace_missing_text_errors
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/edit/test_replace_lines_still_works
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/edit/test_edit_rejects_a_file_never_read_this_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/edit/test_edit_rejects_a_file_changed_since_it_was_read
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/edit/test_read_file_then_edit_file_succeeds
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/edit/test_consecutive_edits_do_not_require_a_re_read_between_them
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [error](../../../../../../../functions/src/llm/tools/trait/ToolResult/error.md)
- [normalize_input](../../../../../../../functions/src/llm/tools/edit/normalize_input.md)
- [check_path](../../../../../../../functions/src/llm/tools/sandbox/check_path.md)
- [validate_file_path](../../../../../../../functions/src/llm/tools/error/validate_file_path.md)
- [check](../../../../../../../functions/src/llm/tools/file_read_cache/FileReadCache/check.md)
- [of](../../../../../../../functions/src/llm/tools/file_read_cache/FileFingerprint/of.md)
- [write](../../../../../../../functions/src/memory/project/write.md)
- [record](../../../../../../../functions/src/llm/tools/file_read_cache/FileReadCache/record.md)
- [success](../../../../../../../functions/src/llm/tools/trait/ToolResult/success.md)
- [display](../../../../../../../functions/src/llm/tools/skill/SkillSource/display.md)

# Called by

- [test_replace_with_explicit_operation_still_works](../../../../../../../functions/src/llm/tools/edit/test_replace_with_explicit_operation_still_works.md)
- [test_qwen_code_and_claude_code_style_payload_works_with_no_operation_field](../../../../../../../functions/src/llm/tools/edit/test_qwen_code_and_claude_code_style_payload_works_with_no_operation_field.md)
- [test_replace_rejects_non_unique_match_by_default](../../../../../../../functions/src/llm/tools/edit/test_replace_rejects_non_unique_match_by_default.md)
- [test_replace_all_true_replaces_every_occurrence](../../../../../../../functions/src/llm/tools/edit/test_replace_all_true_replaces_every_occurrence.md)
- [test_replace_missing_text_errors](../../../../../../../functions/src/llm/tools/edit/test_replace_missing_text_errors.md)
- [test_replace_lines_still_works](../../../../../../../functions/src/llm/tools/edit/test_replace_lines_still_works.md)
- [test_edit_rejects_a_file_never_read_this_session](../../../../../../../functions/src/llm/tools/edit/test_edit_rejects_a_file_never_read_this_session.md)
- [test_edit_rejects_a_file_changed_since_it_was_read](../../../../../../../functions/src/llm/tools/edit/test_edit_rejects_a_file_changed_since_it_was_read.md)
- [test_read_file_then_edit_file_succeeds](../../../../../../../functions/src/llm/tools/edit/test_read_file_then_edit_file_succeeds.md)
- [test_consecutive_edits_do_not_require_a_re_read_between_them](../../../../../../../functions/src/llm/tools/edit/test_consecutive_edits_do_not_require_a_re_read_between_them.md)