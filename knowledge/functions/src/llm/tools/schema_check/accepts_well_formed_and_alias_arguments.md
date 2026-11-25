---
type: Rust Function
title: accepts_well_formed_and_alias_arguments
resource: src/llm/tools/schema_check.rs#L162-L170
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/schema_check/check_arguments
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/schema_check/schema
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn accepts_well_formed_and_alias_arguments()`

# Calls

- [check_arguments](../../../../../functions/src/llm/tools/schema_check/check_arguments.md)
- [schema](../../../../../functions/src/llm/tools/schema_check/schema.md)