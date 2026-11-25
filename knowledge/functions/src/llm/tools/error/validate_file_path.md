---
type: Rust Function
title: validate_file_path
resource: src/llm/tools/error.rs#L120-L149
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/error/validate_path_safety
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/exists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/SkillSource/display
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/edit/EditTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/read/ReadTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn validate_file_path( requested_path: &str, working_directory: &std::path::Path, ) -> std::result::Result<std::path::PathBuf, String>`

# Calls

- [validate_path_safety](../../../../../functions/src/llm/tools/error/validate_path_safety.md)
- [exists](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/exists.md)
- [display](../../../../../functions/src/llm/tools/skill/SkillSource/display.md)

# Called by

- [execute](../../../../../functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/execute.md)
- [execute](../../../../../functions/src/llm/tools/edit/EditTool/tool/execute.md)
- [execute](../../../../../functions/src/llm/tools/read/ReadTool/tool/execute.md)