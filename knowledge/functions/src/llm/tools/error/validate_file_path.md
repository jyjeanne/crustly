---
type: Rust Function
title: validate_file_path
resource: src/llm/tools/error.rs#L120-L149
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/tools/error/validate_path_safety
  called_by:
  - functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/execute
  - functions/src/llm/tools/edit/EditTool/tool/execute
  - functions/src/llm/tools/read/ReadTool/tool/execute
---

# Signature

`pub fn validate_file_path( requested_path: &str, working_directory: &std::path::Path, ) -> std::result::Result<std::path::PathBuf, String>`

# Calls

- [validate_path_safety](../../../../../functions/src/llm/tools/error/validate_path_safety.md)

# Called by

- [execute](../../../../../functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/execute.md)
- [execute](../../../../../functions/src/llm/tools/edit/EditTool/tool/execute.md)
- [execute](../../../../../functions/src/llm/tools/read/ReadTool/tool/execute.md)