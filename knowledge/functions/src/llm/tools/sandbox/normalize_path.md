---
type: Rust Function
title: normalize_path
resource: src/llm/tools/sandbox.rs#L430-L442
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/tools/sandbox/DenyPathPrefixRule/permissionpolicy/evaluate
  - functions/src/llm/tools/sandbox/PathBoundaryRule/check
  - functions/src/llm/tools/sandbox/resolve_existing_prefix
---

# Signature

`fn normalize_path(path: &Path) -> PathBuf`

# Called by

- [evaluate](../../../../../functions/src/llm/tools/sandbox/DenyPathPrefixRule/permissionpolicy/evaluate.md)
- [check](../../../../../functions/src/llm/tools/sandbox/PathBoundaryRule/check.md)
- [resolve_existing_prefix](../../../../../functions/src/llm/tools/sandbox/resolve_existing_prefix.md)