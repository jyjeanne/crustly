---
type: Rust Method
title: with_working_directory
resource: src/llm/tools/trait.rs#L158-L161
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/apply_patch/context
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/doc_parser/context_with_file
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/doc_parser/test_nonexistent_file
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/doc_parser/test_path_outside_working_directory_is_denied
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/edit/context
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/forget_memory/context
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/glob/context
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/grep/test_grep_accepts_glob_alias_for_file_pattern
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/grep/test_pattern_is_regex_by_default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/grep/test_regex_false_still_searches_literally
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/grep/test_search_respects_gitignore
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/notebook/test_add_cell_within_working_directory_succeeds
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/notebook/test_path_outside_working_directory_is_denied
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/plan_tool/acceptance_criteria_are_surfaced_at_start_and_completion
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/plan_tool/completing_without_criteria_warns
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/plan_tool/sparse_plan_calls_execute_end_to_end
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/read/test_read_file
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/read/test_read_file_line_range
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/read/test_read_nonexistent_file
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/read/test_read_file_accepts_file_path_alias
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/read/test_five_concurrent_reads_no_deadlock
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/save_memory/context
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/save_memory/db_context
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/search_memory/setup
    resolved_by: tree-sitter-receiver
    confidence: semantic
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

`pub fn with_working_directory(mut self, dir: std::path::PathBuf) -> Self`

# Called by

- [send_message_with_tools_inner](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [context](../../../../../../functions/src/llm/tools/apply_patch/context.md)
- [context_with_file](../../../../../../functions/src/llm/tools/doc_parser/context_with_file.md)
- [test_nonexistent_file](../../../../../../functions/src/llm/tools/doc_parser/test_nonexistent_file.md)
- [test_path_outside_working_directory_is_denied](../../../../../../functions/src/llm/tools/doc_parser/test_path_outside_working_directory_is_denied.md)
- [context](../../../../../../functions/src/llm/tools/edit/context.md)
- [context](../../../../../../functions/src/llm/tools/forget_memory/context.md)
- [context](../../../../../../functions/src/llm/tools/glob/context.md)
- [test_grep_accepts_glob_alias_for_file_pattern](../../../../../../functions/src/llm/tools/grep/test_grep_accepts_glob_alias_for_file_pattern.md)
- [test_pattern_is_regex_by_default](../../../../../../functions/src/llm/tools/grep/test_pattern_is_regex_by_default.md)
- [test_regex_false_still_searches_literally](../../../../../../functions/src/llm/tools/grep/test_regex_false_still_searches_literally.md)
- [test_search_respects_gitignore](../../../../../../functions/src/llm/tools/grep/test_search_respects_gitignore.md)
- [test_add_cell_within_working_directory_succeeds](../../../../../../functions/src/llm/tools/notebook/test_add_cell_within_working_directory_succeeds.md)
- [test_path_outside_working_directory_is_denied](../../../../../../functions/src/llm/tools/notebook/test_path_outside_working_directory_is_denied.md)
- [acceptance_criteria_are_surfaced_at_start_and_completion](../../../../../../functions/src/llm/tools/plan_tool/acceptance_criteria_are_surfaced_at_start_and_completion.md)
- [completing_without_criteria_warns](../../../../../../functions/src/llm/tools/plan_tool/completing_without_criteria_warns.md)
- [sparse_plan_calls_execute_end_to_end](../../../../../../functions/src/llm/tools/plan_tool/sparse_plan_calls_execute_end_to_end.md)
- [test_read_file](../../../../../../functions/src/llm/tools/read/test_read_file.md)
- [test_read_file_line_range](../../../../../../functions/src/llm/tools/read/test_read_file_line_range.md)
- [test_read_nonexistent_file](../../../../../../functions/src/llm/tools/read/test_read_nonexistent_file.md)
- [test_read_file_accepts_file_path_alias](../../../../../../functions/src/llm/tools/read/test_read_file_accepts_file_path_alias.md)
- [test_five_concurrent_reads_no_deadlock](../../../../../../functions/src/llm/tools/read/test_five_concurrent_reads_no_deadlock.md)
- [context](../../../../../../functions/src/llm/tools/save_memory/context.md)
- [db_context](../../../../../../functions/src/llm/tools/save_memory/db_context.md)
- [setup](../../../../../../functions/src/llm/tools/search_memory/setup.md)
- [test_write_file](../../../../../../functions/src/llm/tools/write/test_write_file.md)
- [test_write_file_with_create_dirs](../../../../../../functions/src/llm/tools/write/test_write_file_with_create_dirs.md)
- [test_write_file_missing_parent_dir](../../../../../../functions/src/llm/tools/write/test_write_file_missing_parent_dir.md)
- [test_write_file_accepts_file_path_alias](../../../../../../functions/src/llm/tools/write/test_write_file_accepts_file_path_alias.md)
- [test_overwrite_existing_file](../../../../../../functions/src/llm/tools/write/test_overwrite_existing_file.md)
- [test_overwrite_rejects_a_file_never_read_this_session](../../../../../../functions/src/llm/tools/write/test_overwrite_rejects_a_file_never_read_this_session.md)
- [test_overwrite_rejects_a_file_changed_since_it_was_read](../../../../../../functions/src/llm/tools/write/test_overwrite_rejects_a_file_changed_since_it_was_read.md)
- [test_creating_a_new_file_needs_no_prior_read](../../../../../../functions/src/llm/tools/write/test_creating_a_new_file_needs_no_prior_read.md)
- [test_write_then_overwrite_does_not_require_a_re_read](../../../../../../functions/src/llm/tools/write/test_write_then_overwrite_does_not_require_a_re_read.md)