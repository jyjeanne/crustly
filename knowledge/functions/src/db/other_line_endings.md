---
type: Rust Function
title: other_line_endings
resource: src/db/mod.rs#L228-L234
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/db/reconcile_line_endings
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn other_line_endings(sql: &str) -> String`

# Called by

- [reconcile_line_endings](../../../functions/src/db/reconcile_line_endings.md)