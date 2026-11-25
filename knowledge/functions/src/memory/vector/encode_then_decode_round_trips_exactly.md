---
type: Rust Function
title: encode_then_decode_round_trips_exactly
resource: src/memory/vector.rs#L116-L121
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/vector/encode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector/decode
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn encode_then_decode_round_trips_exactly()`

# Calls

- [encode](../../../../functions/src/memory/vector/encode.md)
- [decode](../../../../functions/src/memory/vector/decode.md)