---
type: Rust Function
title: only_the_rule_name_is_returned
resource: src/memory/secret_scan.rs#L201-L205
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

`fn only_the_rule_name_is_returned()`

# Calls

- [fake](../../../../functions/src/memory/secret_scan/fake.md)
- [scan](../../../../functions/src/memory/secret_scan/scan.md)