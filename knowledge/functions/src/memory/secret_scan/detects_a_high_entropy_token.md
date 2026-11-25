---
type: Rust Function
title: detects_a_high_entropy_token
resource: src/memory/secret_scan.rs#L178-L182
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/secret_scan/scan
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn detects_a_high_entropy_token()`

# Calls

- [scan](../../../../functions/src/memory/secret_scan/scan.md)