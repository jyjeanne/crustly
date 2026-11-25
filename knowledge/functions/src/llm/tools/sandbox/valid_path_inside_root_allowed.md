---
type: Rust Function
title: valid_path_inside_root_allowed
resource: src/llm/tools/sandbox.rs#L598-L608
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/sandbox/make_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/write
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/sandbox/PathBoundaryRule/permissionpolicy/evaluate
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn valid_path_inside_root_allowed()`

# Calls

- [make_root](../../../../../functions/src/llm/tools/sandbox/make_root.md)
- [write](../../../../../functions/src/memory/project/write.md)
- [evaluate](../../../../../functions/src/llm/tools/sandbox/PathBoundaryRule/permissionpolicy/evaluate.md)