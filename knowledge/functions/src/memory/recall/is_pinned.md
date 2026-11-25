---
type: Rust Function
title: is_pinned
resource: src/memory/recall.rs#L114-L119
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/memory/recall/select
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn is_pinned(memory: &Memory, min_importance: u8) -> bool`

# Called by

- [select](../../../../functions/src/memory/recall/select.md)