---
type: Rust Method
title: display
resource: src/llm/tools/skill.rs#L174-L179
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/cmd_init
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_llama_cpp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/llama_cpp_doctor_findings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_logs
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/system_prompt_tells_the_model_the_working_directory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/pdf_context/augment_message_with_pdf
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/pdf_context/detects_absolute_pdf_token
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/pdf_context/case_insensitive_extension
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/pdf_context/strips_surrounding_quotes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/factory/try_create_llama_cpp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp/worker_loop
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/download_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/bash/BashTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/doc_parser/DocParserTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/edit/EditTool/tool/execute
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
  - target: functions/src/llm/tools/read/ReadTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/sandbox/PathBoundaryRule/check
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/SkillTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/write/WriteTool/tool/execute
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
  - target: functions/src/tui/app/App/handle_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/load_plan_for_viewing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/check_and_load_plan
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/export_plan_to_markdown
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_acceptance_test/a_secret_never_reaches_the_database_file_or_its_wal
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_migration_test/a_database_from_before_memory_upgrades_and_keeps_its_sessions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_minor_fixes_test/a_submodule_is_its_own_project_not_its_superprojects
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn display(&self) -> String`

# Called by

- [cmd_init](../../../../../../functions/src/cli/cmd_init.md)
- [cmd_memory](../../../../../../functions/src/cli/cmd_memory.md)
- [cmd_llama_cpp](../../../../../../functions/src/cli/cmd_llama_cpp.md)
- [llama_cpp_doctor_findings](../../../../../../functions/src/cli/llama_cpp_doctor_findings.md)
- [cmd_logs](../../../../../../functions/src/cli/cmd_logs.md)
- [system_prompt_tells_the_model_the_working_directory](../../../../../../functions/src/llm/agent/service/system_prompt_tells_the_model_the_working_directory.md)
- [augment_message_with_pdf](../../../../../../functions/src/llm/pdf_context/augment_message_with_pdf.md)
- [detects_absolute_pdf_token](../../../../../../functions/src/llm/pdf_context/detects_absolute_pdf_token.md)
- [case_insensitive_extension](../../../../../../functions/src/llm/pdf_context/case_insensitive_extension.md)
- [strips_surrounding_quotes](../../../../../../functions/src/llm/pdf_context/strips_surrounding_quotes.md)
- [try_create_llama_cpp](../../../../../../functions/src/llm/provider/factory/try_create_llama_cpp.md)
- [worker_loop](../../../../../../functions/src/llm/provider/llama_cpp/worker_loop.md)
- [download_model](../../../../../../functions/src/llm/provider/llama_cpp_models/download_model.md)
- [execute](../../../../../../functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/bash/BashTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/edit/EditTool/tool/execute.md)
- [validate_path_safety](../../../../../../functions/src/llm/tools/error/validate_path_safety.md)
- [validate_file_path](../../../../../../functions/src/llm/tools/error/validate_file_path.md)
- [validate_directory_path](../../../../../../functions/src/llm/tools/error/validate_directory_path.md)
- [execute](../../../../../../functions/src/llm/tools/glob/GlobTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/grep/GrepTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/ls/LsTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/notebook/NotebookEditTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/plan_tool/PlanTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/powershell/PowerShellTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/read/ReadTool/tool/execute.md)
- [check](../../../../../../functions/src/llm/tools/sandbox/PathBoundaryRule/check.md)
- [execute](../../../../../../functions/src/llm/tools/skill/SkillTool/tool/execute.md)
- [execute](../../../../../../functions/src/llm/tools/write/WriteTool/tool/execute.md)
- [a_linked_worktree_belongs_to_the_main_repository](../../../../../../functions/src/memory/project/a_linked_worktree_belongs_to_the_main_repository.md)
- [a_worktree_is_found_through_its_commondir_file](../../../../../../functions/src/memory/project/a_worktree_is_found_through_its_commondir_file.md)
- [a_submodule_under_a_worktrees_directory_is_its_own_project](../../../../../../functions/src/memory/project/a_submodule_under_a_worktrees_directory_is_its_own_project.md)
- [handle_event](../../../../../../functions/src/tui/app/App/handle_event.md)
- [load_plan_for_viewing](../../../../../../functions/src/tui/app/App/load_plan_for_viewing.md)
- [check_and_load_plan](../../../../../../functions/src/tui/app/App/check_and_load_plan.md)
- [export_plan_to_markdown](../../../../../../functions/src/tui/app/App/export_plan_to_markdown.md)
- [a_secret_never_reaches_the_database_file_or_its_wal](../../../../../../functions/tests/memory_acceptance_test/a_secret_never_reaches_the_database_file_or_its_wal.md)
- [a_database_from_before_memory_upgrades_and_keeps_its_sessions](../../../../../../functions/tests/memory_migration_test/a_database_from_before_memory_upgrades_and_keeps_its_sessions.md)
- [a_submodule_is_its_own_project_not_its_superprojects](../../../../../../functions/tests/memory_minor_fixes_test/a_submodule_is_its_own_project_not_its_superprojects.md)