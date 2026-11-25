---
type: Rust Function
title: shannon_entropy
resource: src/memory/secret_scan.rs#L74-L93
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/memory/secret_scan/scan
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn shannon_entropy(s: &str) -> f64`

# Called by

- [scan](../../../../functions/src/memory/secret_scan/scan.md)