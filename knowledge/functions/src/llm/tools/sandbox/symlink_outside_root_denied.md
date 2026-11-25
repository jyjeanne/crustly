---
type: Rust Function
title: symlink_outside_root_denied
resource: src/llm/tools/sandbox.rs#L871-L884
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
  - target: functions/src/llm/tools/sandbox/PathBoundaryRule/permissionpolicy/evaluate
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn symlink_outside_root_denied()`

# Calls

- [make_root](../../../../../functions/src/llm/tools/sandbox/make_root.md)
- [evaluate](../../../../../functions/src/llm/tools/sandbox/PathBoundaryRule/permissionpolicy/evaluate.md)