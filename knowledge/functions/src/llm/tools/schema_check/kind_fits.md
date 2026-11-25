---
type: Rust Function
title: kind_fits
resource: src/llm/tools/schema_check.rs#L121-L129
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/tools/schema_check/check_arguments
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn kind_fits(schema_type: &str, value: &Value) -> bool`

# Called by

- [check_arguments](../../../../../functions/src/llm/tools/schema_check/check_arguments.md)