---
type: Rust Function
title: detects_authorization_and_cookie_headers
resource: src/memory/secret_scan.rs#L171-L175
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
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

`fn detects_authorization_and_cookie_headers()`

# Calls

- [fake](../../../../functions/src/memory/secret_scan/fake.md)
- [scan](../../../../functions/src/memory/secret_scan/scan.md)