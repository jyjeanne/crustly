---
type: Rust Method
title: with_temperature
resource: src/llm/provider/types.rs#L203-L206
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/ollama/test_to_ollama_request_maps_common_fields
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/test_llm_request_builder
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/consolidate/MemoryStore/lesson_from_episodes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/extraction/MemoryStore/extract
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn with_temperature(mut self, temperature: f32) -> Self`

# Called by

- [test_to_ollama_request_maps_common_fields](../../../../../../functions/src/llm/provider/ollama/test_to_ollama_request_maps_common_fields.md)
- [test_llm_request_builder](../../../../../../functions/src/llm/provider/types/test_llm_request_builder.md)
- [lesson_from_episodes](../../../../../../functions/src/memory/consolidate/MemoryStore/lesson_from_episodes.md)
- [extract](../../../../../../functions/src/memory/extraction/MemoryStore/extract.md)