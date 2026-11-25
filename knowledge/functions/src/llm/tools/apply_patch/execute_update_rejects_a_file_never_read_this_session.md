---
type: Rust Function
title: execute_update_rejects_a_file_never_read_this_session
resource: src/llm/tools/apply_patch.rs#L866-L886
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/project/write
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn execute_update_rejects_a_file_never_read_this_session()`

# Calls

- [write](../../../../../functions/src/memory/project/write.md)
- [execute](../../../../../functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/execute.md)