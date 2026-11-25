---
type: Rust Function
title: encode
resource: src/memory/vector.rs#L63-L65
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/db/repository/memory/MemoryRepository/put_embedding
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/web_search/WebSearchTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector/encode_then_decode_round_trips_exactly
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector/encoding_is_little_endian
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_vectors_test/a_vector_whose_size_does_not_match_its_record_is_skipped
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn encode(v: &[f32]) -> Vec<u8>`

# Called by

- [put_embedding](../../../../functions/src/db/repository/memory/MemoryRepository/put_embedding.md)
- [execute](../../../../functions/src/llm/tools/web_search/WebSearchTool/tool/execute.md)
- [encode_then_decode_round_trips_exactly](../../../../functions/src/memory/vector/encode_then_decode_round_trips_exactly.md)
- [encoding_is_little_endian](../../../../functions/src/memory/vector/encoding_is_little_endian.md)
- [a_vector_whose_size_does_not_match_its_record_is_skipped](../../../../functions/tests/memory_vectors_test/a_vector_whose_size_does_not_match_its_record_is_skipped.md)