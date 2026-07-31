---
type: Rust Function
title: preview_input
resource: src/llm/tools/registry.rs#L25-L32
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/tools/registry/preview_input_truncates_a_large_payload
  - functions/src/llm/tools/registry/preview_input_truncates_on_char_boundaries
---

# Signature

`fn preview_input(input: &Value) -> String`

# Called by

- [preview_input_truncates_a_large_payload](../../../../../functions/src/llm/tools/registry/preview_input_truncates_a_large_payload.md)
- [preview_input_truncates_on_char_boundaries](../../../../../functions/src/llm/tools/registry/preview_input_truncates_on_char_boundaries.md)