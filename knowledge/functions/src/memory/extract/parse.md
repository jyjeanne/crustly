---
type: Rust Function
title: parse
resource: src/memory/extract.rs#L187-L226
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/extract/check
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn parse( text: &str, session_end: bool, min_importance: u8, max_per_run: usize, ) -> Result<Vec<Candidate>, ParseError>`

# Calls

- [check](../../../../functions/src/memory/extract/check.md)