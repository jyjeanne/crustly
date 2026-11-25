---
type: Rust Function
title: validate_path_safety
resource: src/llm/tools/error.rs#L58-L110
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/skill/SkillSource/display
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/error/validate_file_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/error/validate_directory_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/write/WriteTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn validate_path_safety( requested_path: &str, working_directory: &std::path::Path, ) -> Result<std::path::PathBuf>`

# Calls

- [display](../../../../../functions/src/llm/tools/skill/SkillSource/display.md)

# Called by

- [execute](../../../../../functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/execute.md)
- [validate_file_path](../../../../../functions/src/llm/tools/error/validate_file_path.md)
- [validate_directory_path](../../../../../functions/src/llm/tools/error/validate_directory_path.md)
- [execute](../../../../../functions/src/llm/tools/write/WriteTool/tool/execute.md)