---
type: Rust Function
title: kind_of
resource: src/llm/tools/schema_check.rs#L131-L140
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/tools/schema_check/check_arguments
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn kind_of(value: &Value) -> &'static str`

# Called by

- [check_arguments](../../../../../functions/src/llm/tools/schema_check/check_arguments.md)