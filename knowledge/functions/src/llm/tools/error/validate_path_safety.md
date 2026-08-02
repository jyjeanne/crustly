---
type: Rust Function
title: validate_path_safety
resource: src/llm/tools/error.rs#L58-L110
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/execute
  - functions/src/llm/tools/error/validate_file_path
  - functions/src/llm/tools/error/validate_directory_path
  - functions/src/llm/tools/write/WriteTool/tool/execute
---

# Signature

`pub fn validate_path_safety( requested_path: &str, working_directory: &std::path::Path, ) -> Result<std::path::PathBuf>`

# Called by

- [execute](../../../../../functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/execute.md)
- [validate_file_path](../../../../../functions/src/llm/tools/error/validate_file_path.md)
- [validate_directory_path](../../../../../functions/src/llm/tools/error/validate_directory_path.md)
- [execute](../../../../../functions/src/llm/tools/write/WriteTool/tool/execute.md)