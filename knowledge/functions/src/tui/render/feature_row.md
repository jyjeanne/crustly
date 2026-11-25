---
type: Rust Function
title: feature_row
resource: src/tui/render.rs#L1121-L1132
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/tui/render/help_features
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn feature_row(name: &'static str, desc: &'static str) -> Line<'static>`

# Called by

- [help_features](../../../../functions/src/tui/render/help_features.md)