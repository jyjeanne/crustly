---
type: Rust Function
title: redact_line
resource: src/memory/extract.rs#L78-L84
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/secret_scan/scan
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/extract/redact_and_omit
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn redact_line(line: &str) -> String`

# Calls

- [scan](../../../../functions/src/memory/secret_scan/scan.md)

# Called by

- [redact_and_omit](../../../../functions/src/memory/extract/redact_and_omit.md)