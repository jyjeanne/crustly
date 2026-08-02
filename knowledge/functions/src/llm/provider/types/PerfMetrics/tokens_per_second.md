---
type: Rust Method
title: tokens_per_second
resource: src/llm/provider/types.rs#L335-L338
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/tui/app/App/complete_response
---

# Signature

`pub fn tokens_per_second(&self, output_tokens: u32) -> Option<f64>`

# Called by

- [complete_response](../../../../../../functions/src/tui/app/App/complete_response.md)