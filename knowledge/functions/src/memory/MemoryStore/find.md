---
type: Rust Method
title: find
resource: src/memory/mod.rs#L823-L825
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/MemoryRepository/find_by_prefix
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/resolve_memory
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
  - target: functions/src/config/crabrace/CrabraceIntegration/get_provider
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/compaction/plan_compaction
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/compaction/merge_summaries
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/project_memory/append_progress
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/project_memory/first_line
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/plan_completion_rejection
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/memory_block
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/last_user_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/tool_work_is_remembered_across_requests
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/a_sub_agent_s_answer_comes_back_to_the_parent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/recall_block_reaches_the_model_but_not_the_database
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/without_memory_the_user_turn_is_unchanged
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/anthropic/sse_stream_reassembles_an_event_split_across_chunks
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/parse_nvidia_smi_csv
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/hardware_detect/parse_system_profiler_json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/quantization_hint_from_filename
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/parse_ollama_manifest
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/merge_split_gguf_groups
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/fetch_hf_lfs_sha256
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/mock_resumable_http_server
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_all_local_models_leaves_an_unmatched_mmproj_file_standing_alone
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/find_after
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/parse_native_qwen_tool_calls
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/from_qwen_response
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/find_after_ignores_a_match_before_start
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/find_after_returns_none_when_nothing_matches_after_start
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/find_after_returns_an_absolute_offset_not_a_relative_one
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/tool_call_recovery/fenced_json_blocks
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/aliases/resolve
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/apply_patch/find_subsequence
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/doc_parser/DocParserTool/extract_html_title
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/forget_memory/resolve
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/save_memory/SaveMemoryTool/save_to_store
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/lookup
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/list_skills_discovers_project_local_skills_with_frontmatter
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/list_skills_discovers_legacy_flat_md_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/try_recall
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/semantic/archive_near_duplicates
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanDocument/get_task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanDocument/get_task_mut
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanDocument/next_executable_task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanDocument/next_executable_task_mut
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanDocument/get_task_by_order
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanDocument/get_task_by_order_mut
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/last_assistant_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_chat_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/forget_command
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/check_task_completion
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/export_plan_to_markdown
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/execute_next_plan_task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/fail_current_plan_task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/forget_asks_first_and_deletes_only_on_y
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/highlight/find_syntax
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/integration_test/test_end_to_end_token_usage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_migration_test/a_checksum_recorded_with_other_line_endings_is_accepted_and_updated
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn find(&self, prefix: &str) -> Result<Vec<Memory>>`

# Calls

- [find_by_prefix](../../../../functions/src/db/repository/memory/MemoryRepository/find_by_prefix.md)

# Called by

- [resolve_memory](../../../../functions/src/cli/resolve_memory.md)
- [llama_cpp_doctor_findings_reports_a_writable_models_dir_as_ok](../../../../functions/src/cli/llama_cpp_doctor_findings_reports_a_writable_models_dir_as_ok.md)
- [llama_cpp_doctor_findings_warns_on_a_missing_models_dir](../../../../functions/src/cli/llama_cpp_doctor_findings_warns_on_a_missing_models_dir.md)
- [llama_cpp_doctor_findings_always_reports_detected_hardware](../../../../functions/src/cli/llama_cpp_doctor_findings_always_reports_detected_hardware.md)
- [get_provider](../../../../functions/src/config/crabrace/CrabraceIntegration/get_provider.md)
- [plan_compaction](../../../../functions/src/llm/agent/compaction/plan_compaction.md)
- [merge_summaries](../../../../functions/src/llm/agent/compaction/merge_summaries.md)
- [append_progress](../../../../functions/src/llm/agent/project_memory/append_progress.md)
- [first_line](../../../../functions/src/llm/agent/project_memory/first_line.md)
- [plan_completion_rejection](../../../../functions/src/llm/agent/service/plan_completion_rejection.md)
- [memory_block](../../../../functions/src/llm/agent/service/AgentService/memory_block.md)
- [last_user_text](../../../../functions/src/llm/agent/service/last_user_text.md)
- [tool_work_is_remembered_across_requests](../../../../functions/src/llm/agent/service/tool_work_is_remembered_across_requests.md)
- [a_sub_agent_s_answer_comes_back_to_the_parent](../../../../functions/src/llm/agent/service/a_sub_agent_s_answer_comes_back_to_the_parent.md)
- [recall_block_reaches_the_model_but_not_the_database](../../../../functions/src/llm/agent/service/recall_block_reaches_the_model_but_not_the_database.md)
- [without_memory_the_user_turn_is_unchanged](../../../../functions/src/llm/agent/service/without_memory_the_user_turn_is_unchanged.md)
- [sse_stream_reassembles_an_event_split_across_chunks](../../../../functions/src/llm/provider/anthropic/sse_stream_reassembles_an_event_split_across_chunks.md)
- [parse_nvidia_smi_csv](../../../../functions/src/llm/provider/hardware_detect/parse_nvidia_smi_csv.md)
- [parse_system_profiler_json](../../../../functions/src/llm/provider/hardware_detect/parse_system_profiler_json.md)
- [quantization_hint_from_filename](../../../../functions/src/llm/provider/llama_cpp_models/quantization_hint_from_filename.md)
- [parse_ollama_manifest](../../../../functions/src/llm/provider/llama_cpp_models/parse_ollama_manifest.md)
- [merge_split_gguf_groups](../../../../functions/src/llm/provider/llama_cpp_models/merge_split_gguf_groups.md)
- [fetch_hf_lfs_sha256](../../../../functions/src/llm/provider/llama_cpp_models/fetch_hf_lfs_sha256.md)
- [mock_resumable_http_server](../../../../functions/src/llm/provider/llama_cpp_models/mock_resumable_http_server.md)
- [list_all_local_models_leaves_an_unmatched_mmproj_file_standing_alone](../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models_leaves_an_unmatched_mmproj_file_standing_alone.md)
- [find_after](../../../../functions/src/llm/provider/qwen/find_after.md)
- [parse_native_qwen_tool_calls](../../../../functions/src/llm/provider/qwen/QwenProvider/parse_native_qwen_tool_calls.md)
- [from_qwen_response](../../../../functions/src/llm/provider/qwen/QwenProvider/from_qwen_response.md)
- [find_after_ignores_a_match_before_start](../../../../functions/src/llm/provider/qwen/find_after_ignores_a_match_before_start.md)
- [find_after_returns_none_when_nothing_matches_after_start](../../../../functions/src/llm/provider/qwen/find_after_returns_none_when_nothing_matches_after_start.md)
- [find_after_returns_an_absolute_offset_not_a_relative_one](../../../../functions/src/llm/provider/qwen/find_after_returns_an_absolute_offset_not_a_relative_one.md)
- [fenced_json_blocks](../../../../functions/src/llm/provider/tool_call_recovery/fenced_json_blocks.md)
- [resolve](../../../../functions/src/llm/tools/aliases/resolve.md)
- [find_subsequence](../../../../functions/src/llm/tools/apply_patch/find_subsequence.md)
- [extract_html_title](../../../../functions/src/llm/tools/doc_parser/DocParserTool/extract_html_title.md)
- [resolve](../../../../functions/src/llm/tools/forget_memory/resolve.md)
- [save_to_store](../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/save_to_store.md)
- [lookup](../../../../functions/src/llm/tools/skill/lookup.md)
- [list_skills_discovers_project_local_skills_with_frontmatter](../../../../functions/src/llm/tools/skill/list_skills_discovers_project_local_skills_with_frontmatter.md)
- [list_skills_discovers_legacy_flat_md_files](../../../../functions/src/llm/tools/skill/list_skills_discovers_legacy_flat_md_files.md)
- [try_recall](../../../../functions/src/memory/MemoryStore/try_recall.md)
- [archive_near_duplicates](../../../../functions/src/memory/semantic/archive_near_duplicates.md)
- [get_task](../../../../functions/src/plan/PlanDocument/get_task.md)
- [get_task_mut](../../../../functions/src/plan/PlanDocument/get_task_mut.md)
- [next_executable_task](../../../../functions/src/plan/PlanDocument/next_executable_task.md)
- [next_executable_task_mut](../../../../functions/src/plan/PlanDocument/next_executable_task_mut.md)
- [get_task_by_order](../../../../functions/src/plan/PlanDocument/get_task_by_order.md)
- [get_task_by_order_mut](../../../../functions/src/plan/PlanDocument/get_task_by_order_mut.md)
- [last_assistant_message](../../../../functions/src/tui/app/App/last_assistant_message.md)
- [handle_chat_key](../../../../functions/src/tui/app/App/handle_chat_key.md)
- [forget_command](../../../../functions/src/tui/app/App/forget_command.md)
- [check_task_completion](../../../../functions/src/tui/app/App/check_task_completion.md)
- [export_plan_to_markdown](../../../../functions/src/tui/app/App/export_plan_to_markdown.md)
- [execute_next_plan_task](../../../../functions/src/tui/app/App/execute_next_plan_task.md)
- [fail_current_plan_task](../../../../functions/src/tui/app/App/fail_current_plan_task.md)
- [forget_asks_first_and_deletes_only_on_y](../../../../functions/src/tui/app/forget_asks_first_and_deletes_only_on_y.md)
- [find_syntax](../../../../functions/src/tui/highlight/find_syntax.md)
- [test_end_to_end_token_usage](../../../../functions/tests/integration_test/test_end_to_end_token_usage.md)
- [a_checksum_recorded_with_other_line_endings_is_accepted_and_updated](../../../../functions/tests/memory_migration_test/a_checksum_recorded_with_other_line_endings_is_accepted_and_updated.md)