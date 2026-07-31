---
type: Rust Function
title: check_path
resource: src/llm/tools/sandbox.rs#L406-L425
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/execute
  - functions/src/llm/tools/doc_parser/DocParserTool/tool/execute
  - functions/src/llm/tools/edit/EditTool/tool/execute
  - functions/src/llm/tools/glob/GlobTool/tool/execute
  - functions/src/llm/tools/grep/GrepTool/tool/execute
  - functions/src/llm/tools/ls/LsTool/tool/execute
  - functions/src/llm/tools/notebook/NotebookEditTool/tool/execute
  - functions/src/llm/tools/read/ReadTool/tool/execute
  - functions/src/llm/tools/sandbox/absolute_path_to_nonexistent_file_through_a_symlinked_root_allowed
  - functions/src/llm/tools/write/WriteTool/tool/execute
---

# Signature

`pub fn check_path(raw: &str, root: &Path) -> Result<(), String>`

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
- [execute](../../../../../functions/src/llm/tools/write/WriteTool/tool/execute.md)