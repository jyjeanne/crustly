---
type: Rust Method
title: new
resource: src/llm/tools/trait.rs#L137-L149
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/file_read_cache/FileReadCache/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/agent/run_waits_for_the_answer_and_returns_it
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/agent/without_a_launcher_the_tool_says_so
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/apply_patch/context
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/ask_user/test_auto_approve_returns_placeholder
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/bash/bash_runs_posix_in_the_requested_working_directory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/bash/test_bash_simple_command
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/bash/test_bash_with_exit_code
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/bash/test_bash_invalid_command
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/bash/test_bash_timeout
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/bash/test_bash_accepts_directory_alias
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/bash/test_bash_timeout_field_overrides_context_default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/bash/test_bash_is_background_notes_synchronous_fallback
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
  - target: functions/src/llm/tools/http/execute_denies_cloud_metadata_endpoint
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/http/execute_denies_loopback_address
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
  - target: functions/src/llm/tools/powershell/make_ctx
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
  - target: functions/src/llm/tools/registry/test_execute_tool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/test_execute_nonexistent_tool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/test_execute_requires_approval
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/test_execute_with_auto_approve
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/execute_resolves_an_alias_name_to_the_registered_tool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/execute_reports_not_found_using_the_original_unresolved_name
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/execute_evaluates_policy_against_the_canonical_name_not_the_alias
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
  - target: functions/src/llm/tools/trait/test_execution_context
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/web_fetch/execute_denies_cloud_metadata_endpoint
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/web_fetch/execute_denies_loopback_address
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
  - target: functions/tests/memory_review_fixes_test/forget_is_refused_in_plan_mode_and_deletes_nothing
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(session_id: Uuid) -> Self`

# Calls

- [new](../../../../../../functions/src/llm/tools/file_read_cache/FileReadCache/new.md)

# Called by

- [send_message_with_tools_inner](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [run_waits_for_the_answer_and_returns_it](../../../../../../functions/src/llm/tools/agent/run_waits_for_the_answer_and_returns_it.md)
- [without_a_launcher_the_tool_says_so](../../../../../../functions/src/llm/tools/agent/without_a_launcher_the_tool_says_so.md)
- [context](../../../../../../functions/src/llm/tools/apply_patch/context.md)
- [test_auto_approve_returns_placeholder](../../../../../../functions/src/llm/tools/ask_user/test_auto_approve_returns_placeholder.md)
- [bash_runs_posix_in_the_requested_working_directory](../../../../../../functions/src/llm/tools/bash/bash_runs_posix_in_the_requested_working_directory.md)
- [test_bash_simple_command](../../../../../../functions/src/llm/tools/bash/test_bash_simple_command.md)
- [test_bash_with_exit_code](../../../../../../functions/src/llm/tools/bash/test_bash_with_exit_code.md)
- [test_bash_invalid_command](../../../../../../functions/src/llm/tools/bash/test_bash_invalid_command.md)
- [test_bash_timeout](../../../../../../functions/src/llm/tools/bash/test_bash_timeout.md)
- [test_bash_accepts_directory_alias](../../../../../../functions/src/llm/tools/bash/test_bash_accepts_directory_alias.md)
- [test_bash_timeout_field_overrides_context_default](../../../../../../functions/src/llm/tools/bash/test_bash_timeout_field_overrides_context_default.md)
- [test_bash_is_background_notes_synchronous_fallback](../../../../../../functions/src/llm/tools/bash/test_bash_is_background_notes_synchronous_fallback.md)
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
- [execute_denies_cloud_metadata_endpoint](../../../../../../functions/src/llm/tools/http/execute_denies_cloud_metadata_endpoint.md)
- [execute_denies_loopback_address](../../../../../../functions/src/llm/tools/http/execute_denies_loopback_address.md)
- [test_add_cell_within_working_directory_succeeds](../../../../../../functions/src/llm/tools/notebook/test_add_cell_within_working_directory_succeeds.md)
- [test_path_outside_working_directory_is_denied](../../../../../../functions/src/llm/tools/notebook/test_path_outside_working_directory_is_denied.md)
- [acceptance_criteria_are_surfaced_at_start_and_completion](../../../../../../functions/src/llm/tools/plan_tool/acceptance_criteria_are_surfaced_at_start_and_completion.md)
- [completing_without_criteria_warns](../../../../../../functions/src/llm/tools/plan_tool/completing_without_criteria_warns.md)
- [sparse_plan_calls_execute_end_to_end](../../../../../../functions/src/llm/tools/plan_tool/sparse_plan_calls_execute_end_to_end.md)
- [make_ctx](../../../../../../functions/src/llm/tools/powershell/make_ctx.md)
- [test_read_file](../../../../../../functions/src/llm/tools/read/test_read_file.md)
- [test_read_file_line_range](../../../../../../functions/src/llm/tools/read/test_read_file_line_range.md)
- [test_read_nonexistent_file](../../../../../../functions/src/llm/tools/read/test_read_nonexistent_file.md)
- [test_read_file_accepts_file_path_alias](../../../../../../functions/src/llm/tools/read/test_read_file_accepts_file_path_alias.md)
- [test_five_concurrent_reads_no_deadlock](../../../../../../functions/src/llm/tools/read/test_five_concurrent_reads_no_deadlock.md)
- [test_execute_tool](../../../../../../functions/src/llm/tools/registry/test_execute_tool.md)
- [test_execute_nonexistent_tool](../../../../../../functions/src/llm/tools/registry/test_execute_nonexistent_tool.md)
- [test_execute_requires_approval](../../../../../../functions/src/llm/tools/registry/test_execute_requires_approval.md)
- [test_execute_with_auto_approve](../../../../../../functions/src/llm/tools/registry/test_execute_with_auto_approve.md)
- [execute_resolves_an_alias_name_to_the_registered_tool](../../../../../../functions/src/llm/tools/registry/execute_resolves_an_alias_name_to_the_registered_tool.md)
- [execute_reports_not_found_using_the_original_unresolved_name](../../../../../../functions/src/llm/tools/registry/execute_reports_not_found_using_the_original_unresolved_name.md)
- [execute_evaluates_policy_against_the_canonical_name_not_the_alias](../../../../../../functions/src/llm/tools/registry/execute_evaluates_policy_against_the_canonical_name_not_the_alias.md)
- [context](../../../../../../functions/src/llm/tools/save_memory/context.md)
- [db_context](../../../../../../functions/src/llm/tools/save_memory/db_context.md)
- [setup](../../../../../../functions/src/llm/tools/search_memory/setup.md)
- [test_execution_context](../../../../../../functions/src/llm/tools/trait/test_execution_context.md)
- [execute_denies_cloud_metadata_endpoint](../../../../../../functions/src/llm/tools/web_fetch/execute_denies_cloud_metadata_endpoint.md)
- [execute_denies_loopback_address](../../../../../../functions/src/llm/tools/web_fetch/execute_denies_loopback_address.md)
- [test_write_file](../../../../../../functions/src/llm/tools/write/test_write_file.md)
- [test_write_file_with_create_dirs](../../../../../../functions/src/llm/tools/write/test_write_file_with_create_dirs.md)
- [test_write_file_missing_parent_dir](../../../../../../functions/src/llm/tools/write/test_write_file_missing_parent_dir.md)
- [test_write_file_accepts_file_path_alias](../../../../../../functions/src/llm/tools/write/test_write_file_accepts_file_path_alias.md)
- [test_overwrite_existing_file](../../../../../../functions/src/llm/tools/write/test_overwrite_existing_file.md)
- [test_overwrite_rejects_a_file_never_read_this_session](../../../../../../functions/src/llm/tools/write/test_overwrite_rejects_a_file_never_read_this_session.md)
- [test_overwrite_rejects_a_file_changed_since_it_was_read](../../../../../../functions/src/llm/tools/write/test_overwrite_rejects_a_file_changed_since_it_was_read.md)
- [test_creating_a_new_file_needs_no_prior_read](../../../../../../functions/src/llm/tools/write/test_creating_a_new_file_needs_no_prior_read.md)
- [test_write_then_overwrite_does_not_require_a_re_read](../../../../../../functions/src/llm/tools/write/test_write_then_overwrite_does_not_require_a_re_read.md)
- [forget_is_refused_in_plan_mode_and_deletes_nothing](../../../../../../functions/tests/memory_review_fixes_test/forget_is_refused_in_plan_mode_and_deletes_nothing.md)