---
type: Rust Function
title: decode
resource: src/memory/vector.rs#L69-L83
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/db/repository/memory/MemoryRepository/vectors_for
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector/encode_then_decode_round_trips_exactly
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector/decode_refuses_a_length_that_does_not_match
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn decode(bytes: &[u8], dimensions: usize) -> Result<Vec<f32>, VectorError>`

# Called by

- [vectors_for](../../../../functions/src/db/repository/memory/MemoryRepository/vectors_for.md)
- [encode_then_decode_round_trips_exactly](../../../../functions/src/memory/vector/encode_then_decode_round_trips_exactly.md)
- [decode_refuses_a_length_that_does_not_match](../../../../functions/src/memory/vector/decode_refuses_a_length_that_does_not_match.md)