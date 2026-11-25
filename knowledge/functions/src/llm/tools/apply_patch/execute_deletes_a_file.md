---
type: Rust Function
title: execute_deletes_a_file
resource: src/llm/tools/apply_patch.rs#L711-L724
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
  - target: functions/src/memory/code_knowledge/CodeKnowledge/exists
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn execute_deletes_a_file()`

# Calls

- [write](../../../../../functions/src/memory/project/write.md)
- [execute](../../../../../functions/src/llm/tools/apply_patch/ApplyPatchTool/tool/execute.md)
- [exists](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/exists.md)