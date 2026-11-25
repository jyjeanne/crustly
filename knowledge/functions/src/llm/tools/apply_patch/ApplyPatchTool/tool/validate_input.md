---
type: Rust Method
title: validate_input
resource: src/llm/tools/apply_patch.rs#L336-L341
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/apply_patch/parse_patch
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn validate_input(&self, input: &Value) -> Result<()>`

# Calls

- [parse_patch](../../../../../../../functions/src/llm/tools/apply_patch/parse_patch.md)