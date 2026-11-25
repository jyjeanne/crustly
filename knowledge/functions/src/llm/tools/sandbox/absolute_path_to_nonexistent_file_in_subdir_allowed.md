---
type: Rust Function
title: absolute_path_to_nonexistent_file_in_subdir_allowed
resource: src/llm/tools/sandbox.rs#L535-L544
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/code_knowledge/CodeKnowledge/exists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/sandbox/check_path
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn absolute_path_to_nonexistent_file_in_subdir_allowed()`

# Calls

- [exists](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/exists.md)
- [check_path](../../../../../functions/src/llm/tools/sandbox/check_path.md)