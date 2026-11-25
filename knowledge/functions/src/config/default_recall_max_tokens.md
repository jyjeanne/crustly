---
type: Rust Function
title: default_recall_max_tokens
resource: src/config/mod.rs#L441-L443
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/config/MemoryConfig/default/default
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/MemoryConfig/sanitized
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn default_recall_max_tokens() -> u32`

# Called by

- [default](../../../functions/src/config/MemoryConfig/default/default.md)
- [sanitized](../../../functions/src/config/MemoryConfig/sanitized.md)