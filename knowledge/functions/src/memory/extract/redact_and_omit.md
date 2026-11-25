---
type: Rust Function
title: redact_and_omit
resource: src/memory/extract.rs#L47-L76
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/extract/redact_line
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/extract/transcript
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn redact_and_omit(text: &str) -> String`

# Calls

- [redact_line](../../../../functions/src/memory/extract/redact_line.md)

# Called by

- [transcript](../../../../functions/src/memory/extract/transcript.md)