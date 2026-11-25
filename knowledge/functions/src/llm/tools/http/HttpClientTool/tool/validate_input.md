---
type: Rust Method
title: validate_input
resource: src/llm/tools/http.rs#L137-L169
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/http/parse_method
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn validate_input(&self, input: &Value) -> Result<()>`

# Calls

- [parse_method](../../../../../../../functions/src/llm/tools/http/parse_method.md)