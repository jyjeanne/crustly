---
type: Rust Method
title: exists
resource: src/memory/code_knowledge.rs#L553-L566
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/code_knowledge/CodeKnowledge/call_json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_init
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/llama_cpp_doctor_findings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_logs
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/llama_cpp_doctor_findings_reports_a_writable_models_dir_as_ok
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/load
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/project_config_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/validate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/Database/connect
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/tilde_in_the_database_path_is_expanded_to_home
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/plan_completion_rejection
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/subagents/write_report
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/pdf_context/looks_like_pdf_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/available_space_at
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/download_model_rejects_a_checksum_mismatch_and_cleans_up
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/execute
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
  - target: functions/src/llm/tools/bash/BashTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/doc_parser/DocParserTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/error/validate_path_safety
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/error/validate_file_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/error/validate_directory_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/glob/GlobTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/grep/GrepTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/ls/LsTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/notebook/NotebookEditTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/plan_tool/PlanTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/powershell/PowerShellTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/sandbox/resolve_existing_prefix
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/sandbox/absolute_path_to_nonexistent_file_inside_root_allowed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/sandbox/absolute_path_to_nonexistent_file_in_subdir_allowed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/sandbox/absolute_path_to_nonexistent_file_through_a_symlinked_root_allowed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/save_memory/database_save_is_stored_announced_and_not_written_to_the_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/write/WriteTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/logging/init_debug_logging
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/logging/get_log_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/logging/cleanup_old_logs
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/anchors/MemoryStore/set_anchors
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/anchors/MemoryStore/drifted
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/plan/test_service_export_to_json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/plan/test_service_atomic_json_write
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/check_and_load_plan
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_code_knowledge_test/real_okf_mcp_answers_in_json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/plan_mode_integration_test/test_json_export_import_integration
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn exists(&self, id: &str) -> Option<bool>`

# Calls

- [call_json](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/call_json.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)

# Called by

- [cmd_init](../../../../../functions/src/cli/cmd_init.md)
- [llama_cpp_doctor_findings](../../../../../functions/src/cli/llama_cpp_doctor_findings.md)
- [cmd_logs](../../../../../functions/src/cli/cmd_logs.md)
- [llama_cpp_doctor_findings_reports_a_writable_models_dir_as_ok](../../../../../functions/src/cli/llama_cpp_doctor_findings_reports_a_writable_models_dir_as_ok.md)
- [load](../../../../../functions/src/config/Config/load.md)
- [project_config_path](../../../../../functions/src/config/Config/project_config_path.md)
- [validate](../../../../../functions/src/config/Config/validate.md)
- [connect](../../../../../functions/src/db/Database/connect.md)
- [tilde_in_the_database_path_is_expanded_to_home](../../../../../functions/src/db/tilde_in_the_database_path_is_expanded_to_home.md)
- [plan_completion_rejection](../../../../../functions/src/llm/agent/service/plan_completion_rejection.md)
- [write_report](../../../../../functions/src/llm/agent/service/subagents/write_report.md)
- [looks_like_pdf_path](../../../../../functions/src/llm/pdf_context/looks_like_pdf_path.md)
- [available_space_at](../../../../../functions/src/llm/provider/llama_cpp_models/available_space_at.md)
- [download_model_rejects_a_checksum_mismatch_and_cleans_up](../../../../../functions/src/llm/provider/llama_cpp_models/download_model_rejects_a_checksum_mismatch_and_cleans_up.md)
- [execute](../../../../../functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/execute.md)
- [execute_deletes_a_file](../../../../../functions/src/llm/tools/apply_patch/execute_deletes_a_file.md)
- [execute_renames_via_move_to](../../../../../functions/src/llm/tools/apply_patch/execute_renames_via_move_to.md)
- [execute_applies_multiple_file_ops_in_one_patch](../../../../../functions/src/llm/tools/apply_patch/execute_applies_multiple_file_ops_in_one_patch.md)
- [execute](../../../../../functions/src/llm/tools/bash/BashTool/tool/execute.md)
- [execute](../../../../../functions/src/llm/tools/doc_parser/DocParserTool/tool/execute.md)
- [validate_path_safety](../../../../../functions/src/llm/tools/error/validate_path_safety.md)
- [validate_file_path](../../../../../functions/src/llm/tools/error/validate_file_path.md)
- [validate_directory_path](../../../../../functions/src/llm/tools/error/validate_directory_path.md)
- [execute](../../../../../functions/src/llm/tools/glob/GlobTool/tool/execute.md)
- [execute](../../../../../functions/src/llm/tools/grep/GrepTool/tool/execute.md)
- [execute](../../../../../functions/src/llm/tools/ls/LsTool/tool/execute.md)
- [execute](../../../../../functions/src/llm/tools/notebook/NotebookEditTool/tool/execute.md)
- [execute](../../../../../functions/src/llm/tools/plan_tool/PlanTool/tool/execute.md)
- [execute](../../../../../functions/src/llm/tools/powershell/PowerShellTool/tool/execute.md)
- [resolve_existing_prefix](../../../../../functions/src/llm/tools/sandbox/resolve_existing_prefix.md)
- [absolute_path_to_nonexistent_file_inside_root_allowed](../../../../../functions/src/llm/tools/sandbox/absolute_path_to_nonexistent_file_inside_root_allowed.md)
- [absolute_path_to_nonexistent_file_in_subdir_allowed](../../../../../functions/src/llm/tools/sandbox/absolute_path_to_nonexistent_file_in_subdir_allowed.md)
- [absolute_path_to_nonexistent_file_through_a_symlinked_root_allowed](../../../../../functions/src/llm/tools/sandbox/absolute_path_to_nonexistent_file_through_a_symlinked_root_allowed.md)
- [database_save_is_stored_announced_and_not_written_to_the_file](../../../../../functions/src/llm/tools/save_memory/database_save_is_stored_announced_and_not_written_to_the_file.md)
- [execute](../../../../../functions/src/llm/tools/write/WriteTool/tool/execute.md)
- [init_debug_logging](../../../../../functions/src/logging/init_debug_logging.md)
- [get_log_path](../../../../../functions/src/logging/get_log_path.md)
- [cleanup_old_logs](../../../../../functions/src/logging/cleanup_old_logs.md)
- [set_anchors](../../../../../functions/src/memory/anchors/MemoryStore/set_anchors.md)
- [drifted](../../../../../functions/src/memory/anchors/MemoryStore/drifted.md)
- [test_service_export_to_json](../../../../../functions/src/services/plan/test_service_export_to_json.md)
- [test_service_atomic_json_write](../../../../../functions/src/services/plan/test_service_atomic_json_write.md)
- [check_and_load_plan](../../../../../functions/src/tui/app/App/check_and_load_plan.md)
- [real_okf_mcp_answers_in_json](../../../../../functions/tests/memory_code_knowledge_test/real_okf_mcp_answers_in_json.md)
- [test_json_export_import_integration](../../../../../functions/tests/plan_mode_integration_test/test_json_export_import_integration.md)