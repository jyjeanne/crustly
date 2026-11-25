---
type: Rust Function
title: normalize_path
resource: src/llm/tools/sandbox.rs#L430-L442
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/tools/sandbox/DenyPathPrefixRule/permissionpolicy/evaluate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/sandbox/PathBoundaryRule/check
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/sandbox/resolve_existing_prefix
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn normalize_path(path: &Path) -> PathBuf`

# Called by

- [evaluate](../../../../../functions/src/llm/tools/sandbox/DenyPathPrefixRule/permissionpolicy/evaluate.md)
- [check](../../../../../functions/src/llm/tools/sandbox/PathBoundaryRule/check.md)
- [resolve_existing_prefix](../../../../../functions/src/llm/tools/sandbox/resolve_existing_prefix.md)