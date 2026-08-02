---
type: Rust Function
title: collect_searchable_files
resource: src/llm/tools/grep.rs#L324-L340
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/tools/grep/GrepTool/tool/execute
---

# Signature

`async fn collect_searchable_files(dir: &Path) -> Result<Vec<PathBuf>>`

# Called by

- [execute](../../../../../functions/src/llm/tools/grep/GrepTool/tool/execute.md)