---
type: Rust Function
title: write
resource: src/memory/project.rs#L130-L133
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/benches/parallel_tool_dispatch/make_temp_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/llama_cpp_doctor_findings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/save
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/config_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/migrating_from_pre_modernization_schema_preserves_existing_messages
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/middleware/verification_detects_the_check_command_from_build_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/project_memory/append_progress
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/project_memory/notes_come_from_the_files_present_in_order
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/project_memory/notes_are_truncated_to_the_budget_at_line_ends
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/plan_completion_gate_decision_matrix
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/verification_sends_the_model_back_to_check_its_edit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/verification_is_off_unless_enabled
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/project_notes_are_added_to_the_system_prompt_when_enabled
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/subagents/write_report
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/pdf_context/detects_absolute_pdf_token
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/pdf_context/detects_relative_pdf_token
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/pdf_context/case_insensitive_extension
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/pdf_context/strips_surrounding_quotes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_local_models_only_lists_gguf_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/download_model_resumes_from_an_existing_partial_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/download_model_verifies_checksum_across_a_resumed_download
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/download_model_falls_back_to_a_fresh_start_when_the_server_ignores_range
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_all_local_models_merges_models_dir_and_extra_paths
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_all_local_models_with_no_ollama_dir_does_not_scan_ollama
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/write_fake_ollama_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_ollama_models_skips_a_manifest_whose_blob_is_missing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_all_local_models_collapses_a_symlink_to_an_already_found_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_all_local_models_collapses_a_split_gguf_group
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_all_local_models_handles_a_partial_split_group_honestly
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_all_local_models_pairs_a_base_model_with_its_mmproj_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_all_local_models_leaves_an_unmatched_mmproj_file_standing_alone
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_all_local_models_does_not_pair_when_two_candidates_are_ambiguous
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_all_local_models_does_not_pair_across_directories
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_all_local_models_does_not_group_unrelated_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/apply_patch/execute_updates_an_existing_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/apply_patch/execute_add_file_that_already_exists_fails
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/apply_patch/execute_deletes_a_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/apply_patch/execute_renames_via_move_to
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/apply_patch/execute_applies_multiple_file_ops_in_one_patch
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/apply_patch/execute_is_atomic_across_files_on_failure
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/apply_patch/execute_blocked_in_read_only_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/apply_patch/execute_update_rejects_a_file_never_read_this_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/apply_patch/execute_add_and_delete_need_no_prior_read
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/code_exec/CodeExecTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/context/ContextStore/save
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/doc_parser/test_path_outside_working_directory_is_denied
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/edit/EditTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/edit/test_replace_with_explicit_operation_still_works
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/edit/test_qwen_code_and_claude_code_style_payload_works_with_no_operation_field
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/edit/test_replace_rejects_non_unique_match_by_default
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/edit/test_replace_all_true_replaces_every_occurrence
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/edit/test_replace_missing_text_errors
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/edit/test_line_operation_without_operation_field_is_rejected
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/edit/test_replace_lines_still_works
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/edit/test_edit_rejects_a_file_never_read_this_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/edit/test_edit_rejects_a_file_changed_since_it_was_read
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/edit/test_read_file_then_edit_file_succeeds
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/edit/test_consecutive_edits_do_not_require_a_re_read_between_them
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/glob/test_glob_matches_recursive_pattern
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/glob/test_glob_respects_gitignore
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/glob/test_glob_respects_limit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/grep/test_grep_accepts_glob_alias_for_file_pattern
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/grep/test_pattern_is_regex_by_default
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/grep/test_regex_false_still_searches_literally
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/grep/test_search_respects_gitignore
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/notebook/NotebookEditTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/notebook/test_add_cell_within_working_directory_succeeds
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/notebook/test_path_outside_working_directory_is_denied
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/plan_tool/PlanTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/plan_tool_security_tests/test_validate_symlink_rejection
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/sandbox/valid_path_inside_root_allowed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/save_memory/SaveMemoryTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/list_skills_discovers_project_local_skills_with_frontmatter
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/list_skills_falls_back_to_directory_name_without_frontmatter_name
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/list_skills_discovers_legacy_flat_md_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/list_skills_is_sorted_alphabetically_case_insensitive
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/list_skills_deduplicates_same_name_across_roots
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/resolve_skill_content_prefers_project_file_over_builtin
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/list_skills_lets_project_skill_shadow_builtin_of_same_name
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/task/TaskStore/save
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/todo_write/TodoStore/save
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/write/WriteTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/write/test_overwrite_existing_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/write/test_overwrite_rejects_a_file_never_read_this_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/write/test_overwrite_rejects_a_file_changed_since_it_was_read
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/logging/init_debug_logging
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/a_bundle_is_found_through_okf_toml_or_knowledge
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/detection_is_off_without_a_command_or_a_bundle
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/a_linked_worktree_belongs_to_the_main_repository
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/a_worktree_is_found_through_its_commondir_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/a_submodule_under_a_worktrees_directory_is_its_own_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/the_origin_url_is_read_for_display
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/two_clones_of_one_remote_are_two_projects
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/transfer/import_from_notes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/plan/PlanService/export_to_json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/plan/test_service_import_from_json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/plan/test_service_json_import_invalid_json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/export_plan_to_markdown
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/llama_cpp_model_header_summary_for_path_reads_a_real_header_once
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_code_knowledge_test/setup
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_code_knowledge_test/a_concept_gone_from_a_newer_bundle_is_flagged_and_not_from_an_older_one
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_code_knowledge_test/real_okf_mcp_answers_in_json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_minor_fixes_test/a_submodule_is_its_own_project_not_its_superprojects
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_paths_test/store_and_real_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_transfer_test/from_notes_moves_bullets_keeps_rejected_ones_and_backs_up_first
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn write(path: &Path, text: &str)`

# Called by

- [make_temp_files](../../../../functions/benches/parallel_tool_dispatch/make_temp_files.md)
- [cmd_memory](../../../../functions/src/cli/cmd_memory.md)
- [llama_cpp_doctor_findings](../../../../functions/src/cli/llama_cpp_doctor_findings.md)
- [save](../../../../functions/src/config/Config/save.md)
- [config_file](../../../../functions/src/config/config_file.md)
- [migrating_from_pre_modernization_schema_preserves_existing_messages](../../../../functions/src/db/migrating_from_pre_modernization_schema_preserves_existing_messages.md)
- [verification_detects_the_check_command_from_build_files](../../../../functions/src/llm/agent/middleware/verification_detects_the_check_command_from_build_files.md)
- [append_progress](../../../../functions/src/llm/agent/project_memory/append_progress.md)
- [notes_come_from_the_files_present_in_order](../../../../functions/src/llm/agent/project_memory/notes_come_from_the_files_present_in_order.md)
- [notes_are_truncated_to_the_budget_at_line_ends](../../../../functions/src/llm/agent/project_memory/notes_are_truncated_to_the_budget_at_line_ends.md)
- [plan_completion_gate_decision_matrix](../../../../functions/src/llm/agent/service/plan_completion_gate_decision_matrix.md)
- [verification_sends_the_model_back_to_check_its_edit](../../../../functions/src/llm/agent/service/verification_sends_the_model_back_to_check_its_edit.md)
- [verification_is_off_unless_enabled](../../../../functions/src/llm/agent/service/verification_is_off_unless_enabled.md)
- [project_notes_are_added_to_the_system_prompt_when_enabled](../../../../functions/src/llm/agent/service/project_notes_are_added_to_the_system_prompt_when_enabled.md)
- [write_report](../../../../functions/src/llm/agent/service/subagents/write_report.md)
- [detects_absolute_pdf_token](../../../../functions/src/llm/pdf_context/detects_absolute_pdf_token.md)
- [detects_relative_pdf_token](../../../../functions/src/llm/pdf_context/detects_relative_pdf_token.md)
- [case_insensitive_extension](../../../../functions/src/llm/pdf_context/case_insensitive_extension.md)
- [strips_surrounding_quotes](../../../../functions/src/llm/pdf_context/strips_surrounding_quotes.md)
- [list_local_models_only_lists_gguf_files](../../../../functions/src/llm/provider/llama_cpp_models/list_local_models_only_lists_gguf_files.md)
- [download_model_resumes_from_an_existing_partial_file](../../../../functions/src/llm/provider/llama_cpp_models/download_model_resumes_from_an_existing_partial_file.md)
- [download_model_verifies_checksum_across_a_resumed_download](../../../../functions/src/llm/provider/llama_cpp_models/download_model_verifies_checksum_across_a_resumed_download.md)
- [download_model_falls_back_to_a_fresh_start_when_the_server_ignores_range](../../../../functions/src/llm/provider/llama_cpp_models/download_model_falls_back_to_a_fresh_start_when_the_server_ignores_range.md)
- [list_all_local_models_merges_models_dir_and_extra_paths](../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models_merges_models_dir_and_extra_paths.md)
- [list_all_local_models_with_no_ollama_dir_does_not_scan_ollama](../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models_with_no_ollama_dir_does_not_scan_ollama.md)
- [write_fake_ollama_model](../../../../functions/src/llm/provider/llama_cpp_models/write_fake_ollama_model.md)
- [list_ollama_models_skips_a_manifest_whose_blob_is_missing](../../../../functions/src/llm/provider/llama_cpp_models/list_ollama_models_skips_a_manifest_whose_blob_is_missing.md)
- [list_all_local_models_collapses_a_symlink_to_an_already_found_file](../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models_collapses_a_symlink_to_an_already_found_file.md)
- [list_all_local_models_collapses_a_split_gguf_group](../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models_collapses_a_split_gguf_group.md)
- [list_all_local_models_handles_a_partial_split_group_honestly](../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models_handles_a_partial_split_group_honestly.md)
- [list_all_local_models_pairs_a_base_model_with_its_mmproj_file](../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models_pairs_a_base_model_with_its_mmproj_file.md)
- [list_all_local_models_leaves_an_unmatched_mmproj_file_standing_alone](../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models_leaves_an_unmatched_mmproj_file_standing_alone.md)
- [list_all_local_models_does_not_pair_when_two_candidates_are_ambiguous](../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models_does_not_pair_when_two_candidates_are_ambiguous.md)
- [list_all_local_models_does_not_pair_across_directories](../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models_does_not_pair_across_directories.md)
- [list_all_local_models_does_not_group_unrelated_files](../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models_does_not_group_unrelated_files.md)
- [execute](../../../../functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/execute.md)
- [execute_updates_an_existing_file](../../../../functions/src/llm/tools/apply_patch/execute_updates_an_existing_file.md)
- [execute_add_file_that_already_exists_fails](../../../../functions/src/llm/tools/apply_patch/execute_add_file_that_already_exists_fails.md)
- [execute_deletes_a_file](../../../../functions/src/llm/tools/apply_patch/execute_deletes_a_file.md)
- [execute_renames_via_move_to](../../../../functions/src/llm/tools/apply_patch/execute_renames_via_move_to.md)
- [execute_applies_multiple_file_ops_in_one_patch](../../../../functions/src/llm/tools/apply_patch/execute_applies_multiple_file_ops_in_one_patch.md)
- [execute_is_atomic_across_files_on_failure](../../../../functions/src/llm/tools/apply_patch/execute_is_atomic_across_files_on_failure.md)
- [execute_blocked_in_read_only_mode](../../../../functions/src/llm/tools/apply_patch/execute_blocked_in_read_only_mode.md)
- [execute_update_rejects_a_file_never_read_this_session](../../../../functions/src/llm/tools/apply_patch/execute_update_rejects_a_file_never_read_this_session.md)
- [execute_add_and_delete_need_no_prior_read](../../../../functions/src/llm/tools/apply_patch/execute_add_and_delete_need_no_prior_read.md)
- [execute](../../../../functions/src/llm/tools/code_exec/CodeExecTool/tool/execute.md)
- [save](../../../../functions/src/llm/tools/context/ContextStore/save.md)
- [test_path_outside_working_directory_is_denied](../../../../functions/src/llm/tools/doc_parser/test_path_outside_working_directory_is_denied.md)
- [execute](../../../../functions/src/llm/tools/edit/EditTool/tool/execute.md)
- [test_replace_with_explicit_operation_still_works](../../../../functions/src/llm/tools/edit/test_replace_with_explicit_operation_still_works.md)
- [test_qwen_code_and_claude_code_style_payload_works_with_no_operation_field](../../../../functions/src/llm/tools/edit/test_qwen_code_and_claude_code_style_payload_works_with_no_operation_field.md)
- [test_replace_rejects_non_unique_match_by_default](../../../../functions/src/llm/tools/edit/test_replace_rejects_non_unique_match_by_default.md)
- [test_replace_all_true_replaces_every_occurrence](../../../../functions/src/llm/tools/edit/test_replace_all_true_replaces_every_occurrence.md)
- [test_replace_missing_text_errors](../../../../functions/src/llm/tools/edit/test_replace_missing_text_errors.md)
- [test_line_operation_without_operation_field_is_rejected](../../../../functions/src/llm/tools/edit/test_line_operation_without_operation_field_is_rejected.md)
- [test_replace_lines_still_works](../../../../functions/src/llm/tools/edit/test_replace_lines_still_works.md)
- [test_edit_rejects_a_file_never_read_this_session](../../../../functions/src/llm/tools/edit/test_edit_rejects_a_file_never_read_this_session.md)
- [test_edit_rejects_a_file_changed_since_it_was_read](../../../../functions/src/llm/tools/edit/test_edit_rejects_a_file_changed_since_it_was_read.md)
- [test_read_file_then_edit_file_succeeds](../../../../functions/src/llm/tools/edit/test_read_file_then_edit_file_succeeds.md)
- [test_consecutive_edits_do_not_require_a_re_read_between_them](../../../../functions/src/llm/tools/edit/test_consecutive_edits_do_not_require_a_re_read_between_them.md)
- [test_glob_matches_recursive_pattern](../../../../functions/src/llm/tools/glob/test_glob_matches_recursive_pattern.md)
- [test_glob_respects_gitignore](../../../../functions/src/llm/tools/glob/test_glob_respects_gitignore.md)
- [test_glob_respects_limit](../../../../functions/src/llm/tools/glob/test_glob_respects_limit.md)
- [test_grep_accepts_glob_alias_for_file_pattern](../../../../functions/src/llm/tools/grep/test_grep_accepts_glob_alias_for_file_pattern.md)
- [test_pattern_is_regex_by_default](../../../../functions/src/llm/tools/grep/test_pattern_is_regex_by_default.md)
- [test_regex_false_still_searches_literally](../../../../functions/src/llm/tools/grep/test_regex_false_still_searches_literally.md)
- [test_search_respects_gitignore](../../../../functions/src/llm/tools/grep/test_search_respects_gitignore.md)
- [execute](../../../../functions/src/llm/tools/notebook/NotebookEditTool/tool/execute.md)
- [test_add_cell_within_working_directory_succeeds](../../../../functions/src/llm/tools/notebook/test_add_cell_within_working_directory_succeeds.md)
- [test_path_outside_working_directory_is_denied](../../../../functions/src/llm/tools/notebook/test_path_outside_working_directory_is_denied.md)
- [execute](../../../../functions/src/llm/tools/plan_tool/PlanTool/tool/execute.md)
- [test_validate_symlink_rejection](../../../../functions/src/llm/tools/plan_tool_security_tests/test_validate_symlink_rejection.md)
- [valid_path_inside_root_allowed](../../../../functions/src/llm/tools/sandbox/valid_path_inside_root_allowed.md)
- [execute](../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/tool/execute.md)
- [list_skills_discovers_project_local_skills_with_frontmatter](../../../../functions/src/llm/tools/skill/list_skills_discovers_project_local_skills_with_frontmatter.md)
- [list_skills_falls_back_to_directory_name_without_frontmatter_name](../../../../functions/src/llm/tools/skill/list_skills_falls_back_to_directory_name_without_frontmatter_name.md)
- [list_skills_discovers_legacy_flat_md_files](../../../../functions/src/llm/tools/skill/list_skills_discovers_legacy_flat_md_files.md)
- [list_skills_is_sorted_alphabetically_case_insensitive](../../../../functions/src/llm/tools/skill/list_skills_is_sorted_alphabetically_case_insensitive.md)
- [list_skills_deduplicates_same_name_across_roots](../../../../functions/src/llm/tools/skill/list_skills_deduplicates_same_name_across_roots.md)
- [resolve_skill_content_prefers_project_file_over_builtin](../../../../functions/src/llm/tools/skill/resolve_skill_content_prefers_project_file_over_builtin.md)
- [list_skills_lets_project_skill_shadow_builtin_of_same_name](../../../../functions/src/llm/tools/skill/list_skills_lets_project_skill_shadow_builtin_of_same_name.md)
- [save](../../../../functions/src/llm/tools/task/TaskStore/save.md)
- [save](../../../../functions/src/llm/tools/todo_write/TodoStore/save.md)
- [execute](../../../../functions/src/llm/tools/write/WriteTool/tool/execute.md)
- [test_overwrite_existing_file](../../../../functions/src/llm/tools/write/test_overwrite_existing_file.md)
- [test_overwrite_rejects_a_file_never_read_this_session](../../../../functions/src/llm/tools/write/test_overwrite_rejects_a_file_never_read_this_session.md)
- [test_overwrite_rejects_a_file_changed_since_it_was_read](../../../../functions/src/llm/tools/write/test_overwrite_rejects_a_file_changed_since_it_was_read.md)
- [init_debug_logging](../../../../functions/src/logging/init_debug_logging.md)
- [a_bundle_is_found_through_okf_toml_or_knowledge](../../../../functions/src/memory/code_knowledge/a_bundle_is_found_through_okf_toml_or_knowledge.md)
- [detection_is_off_without_a_command_or_a_bundle](../../../../functions/src/memory/code_knowledge/detection_is_off_without_a_command_or_a_bundle.md)
- [a_linked_worktree_belongs_to_the_main_repository](../../../../functions/src/memory/project/a_linked_worktree_belongs_to_the_main_repository.md)
- [a_worktree_is_found_through_its_commondir_file](../../../../functions/src/memory/project/a_worktree_is_found_through_its_commondir_file.md)
- [a_submodule_under_a_worktrees_directory_is_its_own_project](../../../../functions/src/memory/project/a_submodule_under_a_worktrees_directory_is_its_own_project.md)
- [the_origin_url_is_read_for_display](../../../../functions/src/memory/project/the_origin_url_is_read_for_display.md)
- [two_clones_of_one_remote_are_two_projects](../../../../functions/src/memory/project/two_clones_of_one_remote_are_two_projects.md)
- [import_from_notes](../../../../functions/src/memory/transfer/import_from_notes.md)
- [export_to_json](../../../../functions/src/services/plan/PlanService/export_to_json.md)
- [test_service_import_from_json](../../../../functions/src/services/plan/test_service_import_from_json.md)
- [test_service_json_import_invalid_json](../../../../functions/src/services/plan/test_service_json_import_invalid_json.md)
- [export_plan_to_markdown](../../../../functions/src/tui/app/App/export_plan_to_markdown.md)
- [llama_cpp_model_header_summary_for_path_reads_a_real_header_once](../../../../functions/src/tui/app/llama_cpp_model_header_summary_for_path_reads_a_real_header_once.md)
- [setup](../../../../functions/tests/memory_code_knowledge_test/setup.md)
- [a_concept_gone_from_a_newer_bundle_is_flagged_and_not_from_an_older_one](../../../../functions/tests/memory_code_knowledge_test/a_concept_gone_from_a_newer_bundle_is_flagged_and_not_from_an_older_one.md)
- [real_okf_mcp_answers_in_json](../../../../functions/tests/memory_code_knowledge_test/real_okf_mcp_answers_in_json.md)
- [a_submodule_is_its_own_project_not_its_superprojects](../../../../functions/tests/memory_minor_fixes_test/a_submodule_is_its_own_project_not_its_superprojects.md)
- [store_and_real_project](../../../../functions/tests/memory_paths_test/store_and_real_project.md)
- [from_notes_moves_bullets_keeps_rejected_ones_and_backs_up_first](../../../../functions/tests/memory_transfer_test/from_notes_moves_bullets_keeps_rejected_ones_and_backs_up_first.md)