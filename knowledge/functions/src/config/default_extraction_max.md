---
type: Rust Function
title: default_extraction_max
resource: src/config/mod.rs#L258-L260
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/config/MemoryExtractionConfig/default/default
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/MemoryConfig/sanitized
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn default_extraction_max() -> usize`

# Called by

- [default](../../../functions/src/config/MemoryExtractionConfig/default/default.md)
- [sanitized](../../../functions/src/config/MemoryConfig/sanitized.md)