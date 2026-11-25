---
type: Rust Method
title: contains
resource: src/memory/vector_index.rs#L59-L61
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/llama_cpp_doctor_findings_reports_a_writable_models_dir_as_ok
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/llama_cpp_doctor_findings_warns_on_a_missing_models_dir
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/llama_cpp_doctor_findings_always_reports_detected_hardware
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/llama_cpp_doctor_findings_reports_extra_model_paths_existence
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/llama_cpp_doctor_findings_says_nothing_about_ollama_when_not_opted_in
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/llama_cpp_doctor_findings_reports_build_features
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/llama_cpp_doctor_findings_skips_the_best_fit_finding_when_no_models_present
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/memory_status_reports_semantic_recall_and_extraction_when_on
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/MemoryConfig/sanitized
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/validate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/test_from_layers_reports_the_files_on_an_invalid_value
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/test_from_layers_reports_the_file_with_a_syntax_error
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/test_system_config_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/secrets/test_secret_string_debug
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/secrets/test_secret_string_display
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/secrets/test_secret_string_serialize
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/retry/is_database_locked
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/retry/retry_db_operation
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/context/token_count_bpe_accuracy_rust_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/context/token_count_prose_reasonable
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/middleware/is_file_edit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/middleware/Verification/roundmiddleware/after_tools
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/middleware/limits_finish_with_a_notice_after_the_last_round
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/middleware/verification_nudges_after_an_edit_until_a_check_passes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/project_memory/the_progress_log_grows_and_drops_its_oldest_entries
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/project_memory/recent_work_gives_the_last_entries_oldest_first
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/project_memory/recent_work_drops_the_oldest_entries_first_past_the_cap
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/plan_completion_gate_decision_matrix
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/system_prompt_tells_the_model_the_working_directory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/auto_profile_sends_core_tools_on_a_small_window
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/context_overflow_compacts_and_retries_the_round
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/tool_work_is_remembered_across_requests
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/project_notes_are_added_to_the_system_prompt_when_enabled
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/bad_tool_arguments_get_the_expected_arguments_back
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/a_sub_agent_s_answer_comes_back_to_the_parent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/sub_agents_without_an_approval_prompt_cannot_run_gated_tools
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/sub_agent_tool_calls_are_approved_through_the_parent_s_prompt
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/recall_block_reaches_the_model_but_not_the_database
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/tool_round/AgentService/execute_tool_round
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/tool_profile/ToolSelection/exposes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/error/mentions_context_overflow
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/factory/configure_qwen
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/factory/named_providers_must_be_known_and_configured
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/factory/disabled_anthropic_fallback_fails_with_a_clear_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/factory/test_create_provider_no_credentials
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gemini/test_to_gemini_request_keeps_text_sent_with_tool_results
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gemini/test_build_gemini_error_rate_limit_with_retry_after
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gemini/test_build_gemini_error_rate_limit_without_retry_after
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/parse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/parse_system_profiler_json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/quantization_hint_from_filename
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/filename_suggests_mmproj
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_all_local_models_leaves_an_unmatched_mmproj_file_standing_alone
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/check_disk_space_rejects_when_available_is_below_needed_plus_margin
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/download_model_rejects_a_checksum_mismatch_and_cleans_up
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_all_local_models_merges_two_manifest_tags_sharing_one_blob
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/model_hints/is_vision_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/map_ollama_error
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/model_not_found_error_is_unwrapped_and_actionable
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/to_ollama_request_keeps_text_sent_with_tool_results
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/a_reply_cut_off_before_the_done_chunk_is_an_error
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/openai/test_text_sent_with_tool_results_follows_them
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/default_sampling
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_from_qwen_response_drops_truncated_trailing_hermes_tag_from_display
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_thinking_extraction
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_thinking_extraction_out_of_order_tags_does_not_panic
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_text_sent_with_tool_results_follows_them
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_fallback_parses_bare_json_tool_call
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_fallback_parses_fenced_json_tool_call
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_fallback_does_not_corrupt_unrelated_fenced_code_block
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_stop_words_defined
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/agent/without_a_launcher_the_tool_says_so
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/apply_patch/parse_rejects_missing_begin_marker
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/apply_patch/parse_rejects_missing_end_marker
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/apply_patch/apply_hunks_errors_when_context_not_found
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/bash/is_read_only_command
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/bash/windows_resolves_a_posix_shell_not_cmd
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/code_exec/CodeExecTool/tool/validate_input
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/context/ContextTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/notebook/NotebookEditTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/plan_tool_security_tests/test_validate_path_outside_working_directory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/plan_tool_security_tests/test_validate_filename_pattern
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/plan_tool_security_tests/test_validate_filename_requires_uuid
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/plan_tool_security_tests/test_validate_symlink_rejection
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/plan_tool_security_tests/test_validate_string_empty
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/plan_tool_security_tests/test_validate_string_whitespace_only
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/plan_tool_security_tests/test_validate_string_exceeds_max_length
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/powershell/is_read_only_powershell
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/powershell/execute_blocks_dangerous_command_in_read_only_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/read/test_five_concurrent_reads_no_deadlock
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/save_memory/SaveMemoryTool/save_to_store
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/save_memory/memory_persists_across_different_sessions_in_the_same_directory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/schema_check/rejects_provably_wrong_arguments
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/search_memory/finds_project_and_global_memories_by_words
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/search_memory/scope_and_category_narrow_the_results
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/SkillTool/tool/validate_input
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/SkillTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/resolve_skill_content
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/list_skills_is_sorted_alphabetically_case_insensitive
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/resolve_skill_content_falls_back_to_builtin_review_skill
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/resolve_skill_content_prefers_project_file_over_builtin
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/resolve_skill_content_falls_back_to_builtin_spec_skill
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/ssrf_guard/is_blocked_ipv4
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/task/TaskTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/Tool/requires_approval
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/web_fetch/WebFetchTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/write/WriteTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/logging/test_log_dir_in_crustly_folder
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/logging/debug_filter_is_scoped_to_crustly
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/identifiers
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/parse_code_span
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/bundle_dir
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/exists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/bundle_is_older_than_sources
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/check_capabilities
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/detection_is_off_without_a_command_or_a_bundle
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/consolidate/MemoryStore/consolidate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/embed/ollama_sends_prefixed_input_and_returns_unit_vectors
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/embed/openai_orders_by_index_and_sends_the_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/extract/check
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/save_with
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/try_recall
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/memories_for_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/model/validate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/model/strings_match_the_migration_check_lists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/model/reject_messages_are_actionable
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/paths/validate_glob
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/paths/mentioned_paths
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/plan_tests/test_validate_dependencies_invalid_reference
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/plan_tests/test_validate_dependencies_circular
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/visible_memories
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_chat_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_plan_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/check_task_completion
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/model_download_typing_filters_suggestions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/handle_ollama_pull_finished_success_posts_chat_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/handle_ollama_pull_finished_failure_posts_error_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/handle_ollama_delete_finished_success_removes_from_installed_and_posts_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/handle_ollama_delete_finished_failure_keeps_installed_and_posts_error
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/ctrl_y_copies_last_code_block_when_present
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/ctrl_v_paste_from_clipboard_fails_gracefully_without_panicking
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/slash_review_resolves_to_the_builtin_skill
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/slash_review_with_arguments_appends_them_to_the_skill_prompt
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/slash_spec_resolves_to_the_builtin_skill_with_arguments
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/switch_provider_without_ollama_feature_shows_clear_error
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/switch_provider_with_ollama_feature_swaps_provider_in_place
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/llama_cpp_delete_finished_removes_model_from_list
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/llama_cpp_switch_finished_swaps_provider_in_place
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/llama_cpp_switch_finished_with_error_reports_failure_without_swapping
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/plan_task_error_marks_task_failed_and_stops_auto_execution
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/clear_session_is_refused_while_the_current_session_is_processing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/a_save_during_a_response_is_announced_in_the_chat
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/search_filters_by_words_in_the_content
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/components/ferris/ferris_blinks_and_waves
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/typed_char
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/is_submit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/is_newline
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/highlight/syntect_style_to_ratatui
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/llama_cpp_download/filter_local
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/llama_cpp_download/build_provider_without_feature_reports_a_clear_error
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/ollama_download/filter_suggestions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/prompt_analyzer/PromptAnalyzer/classify_tier
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/prompt_analyzer/hints_skip_tools_that_are_not_available
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/error_scenarios_test/test_error_api_error
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/error_scenarios_test/test_error_rate_limit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/error_scenarios_test/test_error_timeout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/error_scenarios_test/test_error_invalid_response
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/error_scenarios_test/test_error_authentication
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_benchmark_test/TopicEmbedder/embedder/embed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_benchmark_test/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_code_knowledge_test/Scripted/okfserver/call
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/recent_work_from_episodes_keeps_to_its_cap_and_keeps_the_newest
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_migration_test/a_checksum_recorded_with_other_line_endings_is_accepted_and_updated
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_paths_test/an_absolute_path_in_the_message_brings_its_memory_in
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/topic_vector
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_store_test/recall_includes_own_project_and_global_only
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_store_test/recent_work_is_added_only_on_the_first_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_transfer_test/export_then_import_recreates_the_memories_in_a_new_database
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_transfer_test/from_notes_moves_bullets_keeps_rejected_ones_and_backs_up_first
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn contains(&self, id: &Uuid) -> bool`

# Called by

- [cmd_memory](../../../../../functions/src/cli/cmd_memory.md)
- [llama_cpp_doctor_findings_reports_a_writable_models_dir_as_ok](../../../../../functions/src/cli/llama_cpp_doctor_findings_reports_a_writable_models_dir_as_ok.md)
- [llama_cpp_doctor_findings_warns_on_a_missing_models_dir](../../../../../functions/src/cli/llama_cpp_doctor_findings_warns_on_a_missing_models_dir.md)
- [llama_cpp_doctor_findings_always_reports_detected_hardware](../../../../../functions/src/cli/llama_cpp_doctor_findings_always_reports_detected_hardware.md)
- [llama_cpp_doctor_findings_reports_extra_model_paths_existence](../../../../../functions/src/cli/llama_cpp_doctor_findings_reports_extra_model_paths_existence.md)
- [llama_cpp_doctor_findings_says_nothing_about_ollama_when_not_opted_in](../../../../../functions/src/cli/llama_cpp_doctor_findings_says_nothing_about_ollama_when_not_opted_in.md)
- [llama_cpp_doctor_findings_reports_build_features](../../../../../functions/src/cli/llama_cpp_doctor_findings_reports_build_features.md)
- [llama_cpp_doctor_findings_skips_the_best_fit_finding_when_no_models_present](../../../../../functions/src/cli/llama_cpp_doctor_findings_skips_the_best_fit_finding_when_no_models_present.md)
- [memory_status_reports_semantic_recall_and_extraction_when_on](../../../../../functions/src/cli/memory_status_reports_semantic_recall_and_extraction_when_on.md)
- [sanitized](../../../../../functions/src/config/MemoryConfig/sanitized.md)
- [validate](../../../../../functions/src/config/Config/validate.md)
- [test_from_layers_reports_the_files_on_an_invalid_value](../../../../../functions/src/config/test_from_layers_reports_the_files_on_an_invalid_value.md)
- [test_from_layers_reports_the_file_with_a_syntax_error](../../../../../functions/src/config/test_from_layers_reports_the_file_with_a_syntax_error.md)
- [test_system_config_path](../../../../../functions/src/config/test_system_config_path.md)
- [test_secret_string_debug](../../../../../functions/src/config/secrets/test_secret_string_debug.md)
- [test_secret_string_display](../../../../../functions/src/config/secrets/test_secret_string_display.md)
- [test_secret_string_serialize](../../../../../functions/src/config/secrets/test_secret_string_serialize.md)
- [is_database_locked](../../../../../functions/src/db/retry/is_database_locked.md)
- [retry_db_operation](../../../../../functions/src/db/retry/retry_db_operation.md)
- [token_count_bpe_accuracy_rust_file](../../../../../functions/src/llm/agent/context/token_count_bpe_accuracy_rust_file.md)
- [token_count_prose_reasonable](../../../../../functions/src/llm/agent/context/token_count_prose_reasonable.md)
- [is_file_edit](../../../../../functions/src/llm/agent/middleware/is_file_edit.md)
- [after_tools](../../../../../functions/src/llm/agent/middleware/Verification/roundmiddleware/after_tools.md)
- [limits_finish_with_a_notice_after_the_last_round](../../../../../functions/src/llm/agent/middleware/limits_finish_with_a_notice_after_the_last_round.md)
- [verification_nudges_after_an_edit_until_a_check_passes](../../../../../functions/src/llm/agent/middleware/verification_nudges_after_an_edit_until_a_check_passes.md)
- [the_progress_log_grows_and_drops_its_oldest_entries](../../../../../functions/src/llm/agent/project_memory/the_progress_log_grows_and_drops_its_oldest_entries.md)
- [recent_work_gives_the_last_entries_oldest_first](../../../../../functions/src/llm/agent/project_memory/recent_work_gives_the_last_entries_oldest_first.md)
- [recent_work_drops_the_oldest_entries_first_past_the_cap](../../../../../functions/src/llm/agent/project_memory/recent_work_drops_the_oldest_entries_first_past_the_cap.md)
- [plan_completion_gate_decision_matrix](../../../../../functions/src/llm/agent/service/plan_completion_gate_decision_matrix.md)
- [system_prompt_tells_the_model_the_working_directory](../../../../../functions/src/llm/agent/service/system_prompt_tells_the_model_the_working_directory.md)
- [auto_profile_sends_core_tools_on_a_small_window](../../../../../functions/src/llm/agent/service/auto_profile_sends_core_tools_on_a_small_window.md)
- [context_overflow_compacts_and_retries_the_round](../../../../../functions/src/llm/agent/service/context_overflow_compacts_and_retries_the_round.md)
- [tool_work_is_remembered_across_requests](../../../../../functions/src/llm/agent/service/tool_work_is_remembered_across_requests.md)
- [project_notes_are_added_to_the_system_prompt_when_enabled](../../../../../functions/src/llm/agent/service/project_notes_are_added_to_the_system_prompt_when_enabled.md)
- [bad_tool_arguments_get_the_expected_arguments_back](../../../../../functions/src/llm/agent/service/bad_tool_arguments_get_the_expected_arguments_back.md)
- [a_sub_agent_s_answer_comes_back_to_the_parent](../../../../../functions/src/llm/agent/service/a_sub_agent_s_answer_comes_back_to_the_parent.md)
- [sub_agents_without_an_approval_prompt_cannot_run_gated_tools](../../../../../functions/src/llm/agent/service/sub_agents_without_an_approval_prompt_cannot_run_gated_tools.md)
- [sub_agent_tool_calls_are_approved_through_the_parent_s_prompt](../../../../../functions/src/llm/agent/service/sub_agent_tool_calls_are_approved_through_the_parent_s_prompt.md)
- [recall_block_reaches_the_model_but_not_the_database](../../../../../functions/src/llm/agent/service/recall_block_reaches_the_model_but_not_the_database.md)
- [execute_tool_round](../../../../../functions/src/llm/agent/service/tool_round/AgentService/execute_tool_round.md)
- [exposes](../../../../../functions/src/llm/agent/tool_profile/ToolSelection/exposes.md)
- [mentions_context_overflow](../../../../../functions/src/llm/provider/error/mentions_context_overflow.md)
- [configure_qwen](../../../../../functions/src/llm/provider/factory/configure_qwen.md)
- [named_providers_must_be_known_and_configured](../../../../../functions/src/llm/provider/factory/named_providers_must_be_known_and_configured.md)
- [disabled_anthropic_fallback_fails_with_a_clear_message](../../../../../functions/src/llm/provider/factory/disabled_anthropic_fallback_fails_with_a_clear_message.md)
- [test_create_provider_no_credentials](../../../../../functions/src/llm/provider/factory/test_create_provider_no_credentials.md)
- [test_to_gemini_request_keeps_text_sent_with_tool_results](../../../../../functions/src/llm/provider/gemini/test_to_gemini_request_keeps_text_sent_with_tool_results.md)
- [test_build_gemini_error_rate_limit_with_retry_after](../../../../../functions/src/llm/provider/gemini/test_build_gemini_error_rate_limit_with_retry_after.md)
- [test_build_gemini_error_rate_limit_without_retry_after](../../../../../functions/src/llm/provider/gemini/test_build_gemini_error_rate_limit_without_retry_after.md)
- [parse](../../../../../functions/src/llm/provider/gguf_metadata/parse.md)
- [parse_system_profiler_json](../../../../../functions/src/llm/provider/hardware_detect/parse_system_profiler_json.md)
- [quantization_hint_from_filename](../../../../../functions/src/llm/provider/llama_cpp_models/quantization_hint_from_filename.md)
- [filename_suggests_mmproj](../../../../../functions/src/llm/provider/llama_cpp_models/filename_suggests_mmproj.md)
- [list_all_local_models_leaves_an_unmatched_mmproj_file_standing_alone](../../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models_leaves_an_unmatched_mmproj_file_standing_alone.md)
- [check_disk_space_rejects_when_available_is_below_needed_plus_margin](../../../../../functions/src/llm/provider/llama_cpp_models/check_disk_space_rejects_when_available_is_below_needed_plus_margin.md)
- [download_model_rejects_a_checksum_mismatch_and_cleans_up](../../../../../functions/src/llm/provider/llama_cpp_models/download_model_rejects_a_checksum_mismatch_and_cleans_up.md)
- [list_all_local_models_merges_two_manifest_tags_sharing_one_blob](../../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models_merges_two_manifest_tags_sharing_one_blob.md)
- [is_vision_model](../../../../../functions/src/llm/provider/model_hints/is_vision_model.md)
- [map_ollama_error](../../../../../functions/src/llm/provider/ollama/map_ollama_error.md)
- [model_not_found_error_is_unwrapped_and_actionable](../../../../../functions/src/llm/provider/ollama/model_not_found_error_is_unwrapped_and_actionable.md)
- [to_ollama_request_keeps_text_sent_with_tool_results](../../../../../functions/src/llm/provider/ollama/to_ollama_request_keeps_text_sent_with_tool_results.md)
- [a_reply_cut_off_before_the_done_chunk_is_an_error](../../../../../functions/src/llm/provider/ollama/a_reply_cut_off_before_the_done_chunk_is_an_error.md)
- [test_text_sent_with_tool_results_follows_them](../../../../../functions/src/llm/provider/openai/test_text_sent_with_tool_results_follows_them.md)
- [default_sampling](../../../../../functions/src/llm/provider/qwen/QwenProvider/default_sampling.md)
- [test_from_qwen_response_drops_truncated_trailing_hermes_tag_from_display](../../../../../functions/src/llm/provider/qwen/test_from_qwen_response_drops_truncated_trailing_hermes_tag_from_display.md)
- [test_thinking_extraction](../../../../../functions/src/llm/provider/qwen/test_thinking_extraction.md)
- [test_thinking_extraction_out_of_order_tags_does_not_panic](../../../../../functions/src/llm/provider/qwen/test_thinking_extraction_out_of_order_tags_does_not_panic.md)
- [test_text_sent_with_tool_results_follows_them](../../../../../functions/src/llm/provider/qwen/test_text_sent_with_tool_results_follows_them.md)
- [test_fallback_parses_bare_json_tool_call](../../../../../functions/src/llm/provider/qwen/test_fallback_parses_bare_json_tool_call.md)
- [test_fallback_parses_fenced_json_tool_call](../../../../../functions/src/llm/provider/qwen/test_fallback_parses_fenced_json_tool_call.md)
- [test_fallback_does_not_corrupt_unrelated_fenced_code_block](../../../../../functions/src/llm/provider/qwen/test_fallback_does_not_corrupt_unrelated_fenced_code_block.md)
- [test_stop_words_defined](../../../../../functions/src/llm/provider/qwen/test_stop_words_defined.md)
- [without_a_launcher_the_tool_says_so](../../../../../functions/src/llm/tools/agent/without_a_launcher_the_tool_says_so.md)
- [parse_rejects_missing_begin_marker](../../../../../functions/src/llm/tools/apply_patch/parse_rejects_missing_begin_marker.md)
- [parse_rejects_missing_end_marker](../../../../../functions/src/llm/tools/apply_patch/parse_rejects_missing_end_marker.md)
- [apply_hunks_errors_when_context_not_found](../../../../../functions/src/llm/tools/apply_patch/apply_hunks_errors_when_context_not_found.md)
- [is_read_only_command](../../../../../functions/src/llm/tools/bash/is_read_only_command.md)
- [windows_resolves_a_posix_shell_not_cmd](../../../../../functions/src/llm/tools/bash/windows_resolves_a_posix_shell_not_cmd.md)
- [validate_input](../../../../../functions/src/llm/tools/code_exec/CodeExecTool/tool/validate_input.md)
- [execute](../../../../../functions/src/llm/tools/context/ContextTool/tool/execute.md)
- [execute](../../../../../functions/src/llm/tools/notebook/NotebookEditTool/tool/execute.md)
- [test_validate_path_outside_working_directory](../../../../../functions/src/llm/tools/plan_tool_security_tests/test_validate_path_outside_working_directory.md)
- [test_validate_filename_pattern](../../../../../functions/src/llm/tools/plan_tool_security_tests/test_validate_filename_pattern.md)
- [test_validate_filename_requires_uuid](../../../../../functions/src/llm/tools/plan_tool_security_tests/test_validate_filename_requires_uuid.md)
- [test_validate_symlink_rejection](../../../../../functions/src/llm/tools/plan_tool_security_tests/test_validate_symlink_rejection.md)
- [test_validate_string_empty](../../../../../functions/src/llm/tools/plan_tool_security_tests/test_validate_string_empty.md)
- [test_validate_string_whitespace_only](../../../../../functions/src/llm/tools/plan_tool_security_tests/test_validate_string_whitespace_only.md)
- [test_validate_string_exceeds_max_length](../../../../../functions/src/llm/tools/plan_tool_security_tests/test_validate_string_exceeds_max_length.md)
- [is_read_only_powershell](../../../../../functions/src/llm/tools/powershell/is_read_only_powershell.md)
- [execute_blocks_dangerous_command_in_read_only_mode](../../../../../functions/src/llm/tools/powershell/execute_blocks_dangerous_command_in_read_only_mode.md)
- [test_five_concurrent_reads_no_deadlock](../../../../../functions/src/llm/tools/read/test_five_concurrent_reads_no_deadlock.md)
- [save_to_store](../../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/save_to_store.md)
- [memory_persists_across_different_sessions_in_the_same_directory](../../../../../functions/src/llm/tools/save_memory/memory_persists_across_different_sessions_in_the_same_directory.md)
- [rejects_provably_wrong_arguments](../../../../../functions/src/llm/tools/schema_check/rejects_provably_wrong_arguments.md)
- [finds_project_and_global_memories_by_words](../../../../../functions/src/llm/tools/search_memory/finds_project_and_global_memories_by_words.md)
- [scope_and_category_narrow_the_results](../../../../../functions/src/llm/tools/search_memory/scope_and_category_narrow_the_results.md)
- [validate_input](../../../../../functions/src/llm/tools/skill/SkillTool/tool/validate_input.md)
- [execute](../../../../../functions/src/llm/tools/skill/SkillTool/tool/execute.md)
- [resolve_skill_content](../../../../../functions/src/llm/tools/skill/resolve_skill_content.md)
- [list_skills_is_sorted_alphabetically_case_insensitive](../../../../../functions/src/llm/tools/skill/list_skills_is_sorted_alphabetically_case_insensitive.md)
- [resolve_skill_content_falls_back_to_builtin_review_skill](../../../../../functions/src/llm/tools/skill/resolve_skill_content_falls_back_to_builtin_review_skill.md)
- [resolve_skill_content_prefers_project_file_over_builtin](../../../../../functions/src/llm/tools/skill/resolve_skill_content_prefers_project_file_over_builtin.md)
- [resolve_skill_content_falls_back_to_builtin_spec_skill](../../../../../functions/src/llm/tools/skill/resolve_skill_content_falls_back_to_builtin_spec_skill.md)
- [is_blocked_ipv4](../../../../../functions/src/llm/tools/ssrf_guard/is_blocked_ipv4.md)
- [execute](../../../../../functions/src/llm/tools/task/TaskTool/tool/execute.md)
- [requires_approval](../../../../../functions/src/llm/tools/trait/Tool/requires_approval.md)
- [execute](../../../../../functions/src/llm/tools/web_fetch/WebFetchTool/tool/execute.md)
- [execute](../../../../../functions/src/llm/tools/write/WriteTool/tool/execute.md)
- [test_log_dir_in_crustly_folder](../../../../../functions/src/logging/test_log_dir_in_crustly_folder.md)
- [debug_filter_is_scoped_to_crustly](../../../../../functions/src/logging/debug_filter_is_scoped_to_crustly.md)
- [identifiers](../../../../../functions/src/memory/code_knowledge/identifiers.md)
- [parse_code_span](../../../../../functions/src/memory/code_knowledge/parse_code_span.md)
- [bundle_dir](../../../../../functions/src/memory/code_knowledge/bundle_dir.md)
- [exists](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/exists.md)
- [bundle_is_older_than_sources](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/bundle_is_older_than_sources.md)
- [check_capabilities](../../../../../functions/src/memory/code_knowledge/check_capabilities.md)
- [detection_is_off_without_a_command_or_a_bundle](../../../../../functions/src/memory/code_knowledge/detection_is_off_without_a_command_or_a_bundle.md)
- [consolidate](../../../../../functions/src/memory/consolidate/MemoryStore/consolidate.md)
- [ollama_sends_prefixed_input_and_returns_unit_vectors](../../../../../functions/src/memory/embed/ollama_sends_prefixed_input_and_returns_unit_vectors.md)
- [openai_orders_by_index_and_sends_the_key](../../../../../functions/src/memory/embed/openai_orders_by_index_and_sends_the_key.md)
- [check](../../../../../functions/src/memory/extract/check.md)
- [save_with](../../../../../functions/src/memory/MemoryStore/save_with.md)
- [try_recall](../../../../../functions/src/memory/MemoryStore/try_recall.md)
- [memories_for_path](../../../../../functions/src/memory/MemoryStore/memories_for_path.md)
- [validate](../../../../../functions/src/memory/model/validate.md)
- [strings_match_the_migration_check_lists](../../../../../functions/src/memory/model/strings_match_the_migration_check_lists.md)
- [reject_messages_are_actionable](../../../../../functions/src/memory/model/reject_messages_are_actionable.md)
- [validate_glob](../../../../../functions/src/memory/paths/validate_glob.md)
- [mentioned_paths](../../../../../functions/src/memory/paths/mentioned_paths.md)
- [test_validate_dependencies_invalid_reference](../../../../../functions/src/plan/plan_tests/test_validate_dependencies_invalid_reference.md)
- [test_validate_dependencies_circular](../../../../../functions/src/plan/plan_tests/test_validate_dependencies_circular.md)
- [visible_memories](../../../../../functions/src/tui/app/visible_memories.md)
- [handle_chat_key](../../../../../functions/src/tui/app/App/handle_chat_key.md)
- [handle_plan_key](../../../../../functions/src/tui/app/App/handle_plan_key.md)
- [check_task_completion](../../../../../functions/src/tui/app/App/check_task_completion.md)
- [model_download_typing_filters_suggestions](../../../../../functions/src/tui/app/model_download_typing_filters_suggestions.md)
- [handle_ollama_pull_finished_success_posts_chat_message](../../../../../functions/src/tui/app/handle_ollama_pull_finished_success_posts_chat_message.md)
- [handle_ollama_pull_finished_failure_posts_error_message](../../../../../functions/src/tui/app/handle_ollama_pull_finished_failure_posts_error_message.md)
- [handle_ollama_delete_finished_success_removes_from_installed_and_posts_message](../../../../../functions/src/tui/app/handle_ollama_delete_finished_success_removes_from_installed_and_posts_message.md)
- [handle_ollama_delete_finished_failure_keeps_installed_and_posts_error](../../../../../functions/src/tui/app/handle_ollama_delete_finished_failure_keeps_installed_and_posts_error.md)
- [ctrl_y_copies_last_code_block_when_present](../../../../../functions/src/tui/app/ctrl_y_copies_last_code_block_when_present.md)
- [ctrl_v_paste_from_clipboard_fails_gracefully_without_panicking](../../../../../functions/src/tui/app/ctrl_v_paste_from_clipboard_fails_gracefully_without_panicking.md)
- [slash_review_resolves_to_the_builtin_skill](../../../../../functions/src/tui/app/slash_review_resolves_to_the_builtin_skill.md)
- [slash_review_with_arguments_appends_them_to_the_skill_prompt](../../../../../functions/src/tui/app/slash_review_with_arguments_appends_them_to_the_skill_prompt.md)
- [slash_spec_resolves_to_the_builtin_skill_with_arguments](../../../../../functions/src/tui/app/slash_spec_resolves_to_the_builtin_skill_with_arguments.md)
- [switch_provider_without_ollama_feature_shows_clear_error](../../../../../functions/src/tui/app/switch_provider_without_ollama_feature_shows_clear_error.md)
- [switch_provider_with_ollama_feature_swaps_provider_in_place](../../../../../functions/src/tui/app/switch_provider_with_ollama_feature_swaps_provider_in_place.md)
- [llama_cpp_delete_finished_removes_model_from_list](../../../../../functions/src/tui/app/llama_cpp_delete_finished_removes_model_from_list.md)
- [llama_cpp_switch_finished_swaps_provider_in_place](../../../../../functions/src/tui/app/llama_cpp_switch_finished_swaps_provider_in_place.md)
- [llama_cpp_switch_finished_with_error_reports_failure_without_swapping](../../../../../functions/src/tui/app/llama_cpp_switch_finished_with_error_reports_failure_without_swapping.md)
- [plan_task_error_marks_task_failed_and_stops_auto_execution](../../../../../functions/src/tui/app/plan_task_error_marks_task_failed_and_stops_auto_execution.md)
- [clear_session_is_refused_while_the_current_session_is_processing](../../../../../functions/src/tui/app/clear_session_is_refused_while_the_current_session_is_processing.md)
- [a_save_during_a_response_is_announced_in_the_chat](../../../../../functions/src/tui/app/a_save_during_a_response_is_announced_in_the_chat.md)
- [search_filters_by_words_in_the_content](../../../../../functions/src/tui/app/search_filters_by_words_in_the_content.md)
- [ferris_blinks_and_waves](../../../../../functions/src/tui/components/ferris/ferris_blinks_and_waves.md)
- [typed_char](../../../../../functions/src/tui/events/typed_char.md)
- [is_submit](../../../../../functions/src/tui/events/is_submit.md)
- [is_newline](../../../../../functions/src/tui/events/is_newline.md)
- [syntect_style_to_ratatui](../../../../../functions/src/tui/highlight/syntect_style_to_ratatui.md)
- [filter_local](../../../../../functions/src/tui/llama_cpp_download/filter_local.md)
- [build_provider_without_feature_reports_a_clear_error](../../../../../functions/src/tui/llama_cpp_download/build_provider_without_feature_reports_a_clear_error.md)
- [filter_suggestions](../../../../../functions/src/tui/ollama_download/filter_suggestions.md)
- [classify_tier](../../../../../functions/src/tui/prompt_analyzer/PromptAnalyzer/classify_tier.md)
- [hints_skip_tools_that_are_not_available](../../../../../functions/src/tui/prompt_analyzer/hints_skip_tools_that_are_not_available.md)
- [test_error_api_error](../../../../../functions/tests/error_scenarios_test/test_error_api_error.md)
- [test_error_rate_limit](../../../../../functions/tests/error_scenarios_test/test_error_rate_limit.md)
- [test_error_timeout](../../../../../functions/tests/error_scenarios_test/test_error_timeout.md)
- [test_error_invalid_response](../../../../../functions/tests/error_scenarios_test/test_error_invalid_response.md)
- [test_error_authentication](../../../../../functions/tests/error_scenarios_test/test_error_authentication.md)
- [embed](../../../../../functions/tests/memory_benchmark_test/TopicEmbedder/embedder/embed.md)
- [run](../../../../../functions/tests/memory_benchmark_test/run.md)
- [call](../../../../../functions/tests/memory_code_knowledge_test/Scripted/okfserver/call.md)
- [recent_work_from_episodes_keeps_to_its_cap_and_keeps_the_newest](../../../../../functions/tests/memory_extraction_test/recent_work_from_episodes_keeps_to_its_cap_and_keeps_the_newest.md)
- [a_checksum_recorded_with_other_line_endings_is_accepted_and_updated](../../../../../functions/tests/memory_migration_test/a_checksum_recorded_with_other_line_endings_is_accepted_and_updated.md)
- [an_absolute_path_in_the_message_brings_its_memory_in](../../../../../functions/tests/memory_paths_test/an_absolute_path_in_the_message_brings_its_memory_in.md)
- [topic_vector](../../../../../functions/tests/memory_semantic_test/topic_vector.md)
- [recall_includes_own_project_and_global_only](../../../../../functions/tests/memory_store_test/recall_includes_own_project_and_global_only.md)
- [recent_work_is_added_only_on_the_first_request](../../../../../functions/tests/memory_store_test/recent_work_is_added_only_on_the_first_request.md)
- [export_then_import_recreates_the_memories_in_a_new_database](../../../../../functions/tests/memory_transfer_test/export_then_import_recreates_the_memories_in_a_new_database.md)
- [from_notes_moves_bullets_keeps_rejected_ones_and_backs_up_first](../../../../../functions/tests/memory_transfer_test/from_notes_moves_bullets_keeps_rejected_ones_and_backs_up_first.md)