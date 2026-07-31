---
type: Rust Method
title: list_recursive
resource: src/llm/tools/ls.rs#L215-L258
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/tools/ls/LsTool/tool/execute
---

# Signature

`fn list_recursive<'a>( path: &'a PathBuf, input: &'a LsInput, output: &'a mut String, depth: usize, ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send + 'a>>`

# Called by

- [execute](../../../../../../functions/src/llm/tools/ls/LsTool/tool/execute.md)