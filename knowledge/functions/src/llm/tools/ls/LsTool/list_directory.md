---
type: Rust Method
title: list_directory
resource: src/llm/tools/ls.rs#L138-L213
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/tools/ls/LsTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn list_directory( &self, path: &Path, input: &LsInput, output: &mut String, ) -> Result<()>`

# Called by

- [execute](../../../../../../functions/src/llm/tools/ls/LsTool/tool/execute.md)