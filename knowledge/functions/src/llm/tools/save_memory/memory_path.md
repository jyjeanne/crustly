---
type: Rust Function
title: memory_path
resource: src/llm/tools/save_memory.rs#L178-L180
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/tools/save_memory/SaveMemoryTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn memory_path(working_directory: &Path) -> PathBuf`

# Called by

- [execute](../../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/tool/execute.md)