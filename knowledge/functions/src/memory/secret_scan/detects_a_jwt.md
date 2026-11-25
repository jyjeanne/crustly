---
type: Rust Function
title: detects_a_jwt
resource: src/memory/secret_scan.rs#L141-L150
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/secret_scan/fake
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/secret_scan/scan
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn detects_a_jwt()`

# Calls

- [fake](../../../../functions/src/memory/secret_scan/fake.md)
- [scan](../../../../functions/src/memory/secret_scan/scan.md)