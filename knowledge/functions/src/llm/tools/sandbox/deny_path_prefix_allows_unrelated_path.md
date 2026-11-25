---
type: Rust Function
title: deny_path_prefix_allows_unrelated_path
resource: src/llm/tools/sandbox.rs#L858-L867
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/sandbox/make_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/sandbox/DenyPathPrefixRule/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/sandbox/DenyPathPrefixRule/permissionpolicy/evaluate
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn deny_path_prefix_allows_unrelated_path()`

# Calls

- [make_root](../../../../../functions/src/llm/tools/sandbox/make_root.md)
- [new](../../../../../functions/src/llm/tools/sandbox/DenyPathPrefixRule/new.md)
- [evaluate](../../../../../functions/src/llm/tools/sandbox/DenyPathPrefixRule/permissionpolicy/evaluate.md)