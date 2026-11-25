---
type: Rust Function
title: check_path
resource: src/llm/tools/sandbox.rs#L406-L425
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/sandbox/PathBoundaryRule/permissionpolicy/evaluate
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/sandbox/PolicyDecision/is_permitted
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/doc_parser/DocParserTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/edit/EditTool/tool/execute
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
  - target: functions/src/llm/tools/read/ReadTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/sandbox/absolute_path_to_nonexistent_file_through_a_symlinked_root_allowed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/sandbox/absolute_path_to_nonexistent_file_inside_root_allowed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/sandbox/absolute_path_to_nonexistent_file_in_subdir_allowed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/sandbox/absolute_path_to_nonexistent_file_outside_root_still_denied
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/write/WriteTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn check_path(raw: &str, root: &Path) -> Result<(), String>`

# Calls

- [evaluate](../../../../../functions/src/llm/tools/sandbox/PathBoundaryRule/permissionpolicy/evaluate.md)
- [is_permitted](../../../../../functions/src/llm/tools/sandbox/PolicyDecision/is_permitted.md)

# Called by

- [execute](../../../../../functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/execute.md)
- [execute](../../../../../functions/src/llm/tools/doc_parser/DocParserTool/tool/execute.md)
- [execute](../../../../../functions/src/llm/tools/edit/EditTool/tool/execute.md)
- [execute](../../../../../functions/src/llm/tools/glob/GlobTool/tool/execute.md)
- [execute](../../../../../functions/src/llm/tools/grep/GrepTool/tool/execute.md)
- [execute](../../../../../functions/src/llm/tools/ls/LsTool/tool/execute.md)
- [execute](../../../../../functions/src/llm/tools/notebook/NotebookEditTool/tool/execute.md)
- [execute](../../../../../functions/src/llm/tools/read/ReadTool/tool/execute.md)
- [absolute_path_to_nonexistent_file_through_a_symlinked_root_allowed](../../../../../functions/src/llm/tools/sandbox/absolute_path_to_nonexistent_file_through_a_symlinked_root_allowed.md)
- [absolute_path_to_nonexistent_file_inside_root_allowed](../../../../../functions/src/llm/tools/sandbox/absolute_path_to_nonexistent_file_inside_root_allowed.md)
- [absolute_path_to_nonexistent_file_in_subdir_allowed](../../../../../functions/src/llm/tools/sandbox/absolute_path_to_nonexistent_file_in_subdir_allowed.md)
- [absolute_path_to_nonexistent_file_outside_root_still_denied](../../../../../functions/src/llm/tools/sandbox/absolute_path_to_nonexistent_file_outside_root_still_denied.md)
- [execute](../../../../../functions/src/llm/tools/write/WriteTool/tool/execute.md)