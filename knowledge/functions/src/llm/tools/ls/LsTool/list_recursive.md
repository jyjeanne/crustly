---
type: Rust Method
title: list_recursive
resource: src/llm/tools/ls.rs#L215-L258
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/ls/LsTool/list_recursive
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/ls/LsTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/ls/LsTool/list_recursive
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn list_recursive<'a>( path: &'a PathBuf, input: &'a LsInput, output: &'a mut String, depth: usize, ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send + 'a>>`

# Calls

- [list_recursive](../../../../../../functions/src/llm/tools/ls/LsTool/list_recursive.md)

# Called by

- [execute](../../../../../../functions/src/llm/tools/ls/LsTool/tool/execute.md)
- [list_recursive](../../../../../../functions/src/llm/tools/ls/LsTool/list_recursive.md)