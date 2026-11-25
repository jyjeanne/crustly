---
type: Rust Function
title: detects_credentials_in_a_url
resource: src/memory/secret_scan.rs#L153-L156
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

`fn detects_credentials_in_a_url()`

# Calls

- [fake](../../../../functions/src/memory/secret_scan/fake.md)
- [scan](../../../../functions/src/memory/secret_scan/scan.md)