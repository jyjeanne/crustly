---
type: Rust Method
title: validate_input
resource: src/llm/tools/edit.rs#L206-L210
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/edit/normalize_input
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn validate_input(&self, input: &Value) -> Result<()>`

# Calls

- [normalize_input](../../../../../../../functions/src/llm/tools/edit/normalize_input.md)