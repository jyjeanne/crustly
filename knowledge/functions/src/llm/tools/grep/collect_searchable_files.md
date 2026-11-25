---
type: Rust Function
title: collect_searchable_files
resource: src/llm/tools/grep.rs#L324-L340
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/subagents/ChildTemplate/build
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/grep/GrepTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn collect_searchable_files(dir: &Path) -> Result<Vec<PathBuf>>`

# Calls

- [build](../../../../../functions/src/llm/agent/service/subagents/ChildTemplate/build.md)

# Called by

- [execute](../../../../../functions/src/llm/tools/grep/GrepTool/tool/execute.md)