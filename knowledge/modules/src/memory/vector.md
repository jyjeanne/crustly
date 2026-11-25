---
type: Rust Module
title: vector
resource: src/memory/vector.rs#L1-L156
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/std-fmt
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [VectorError](../../../classes/src/memory/vector/VectorError.md)
- [fmt](../../../functions/src/memory/vector/VectorError/fmt-display/fmt.md)
- [normalize](../../../functions/src/memory/vector/normalize.md)
- [encode](../../../functions/src/memory/vector/encode.md)
- [decode](../../../functions/src/memory/vector/decode.md)
- [similarity](../../../functions/src/memory/vector/similarity.md)
- [normalized_vectors_have_unit_length](../../../functions/src/memory/vector/normalized_vectors_have_unit_length.md)
- [zero_and_empty_vectors_are_refused](../../../functions/src/memory/vector/zero_and_empty_vectors_are_refused.md)
- [encode_then_decode_round_trips_exactly](../../../functions/src/memory/vector/encode_then_decode_round_trips_exactly.md)
- [encoding_is_little_endian](../../../functions/src/memory/vector/encoding_is_little_endian.md)
- [decode_refuses_a_length_that_does_not_match](../../../functions/src/memory/vector/decode_refuses_a_length_that_does_not_match.md)
- [similarity_is_the_dot_product_of_unit_vectors](../../../functions/src/memory/vector/similarity_is_the_dot_product_of_unit_vectors.md)
- [similarity_refuses_different_dimensions](../../../functions/src/memory/vector/similarity_refuses_different_dimensions.md)

# Imports

- `std::fmt`
- `super::*`

# Member of

- [crustly](../../../packages/crustly.md)