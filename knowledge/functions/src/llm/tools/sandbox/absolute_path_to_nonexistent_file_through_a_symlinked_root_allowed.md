---
type: Rust Function
title: absolute_path_to_nonexistent_file_through_a_symlinked_root_allowed
resource: src/llm/tools/sandbox.rs#L575-L595
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/sandbox/check_path
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn absolute_path_to_nonexistent_file_through_a_symlinked_root_allowed()`

# Calls

- [check_path](../../../../../functions/src/llm/tools/sandbox/check_path.md)