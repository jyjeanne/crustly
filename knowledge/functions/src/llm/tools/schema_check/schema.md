---
type: Rust Function
title: schema
resource: src/llm/tools/schema_check.rs#L147-L159
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/tools/schema_check/rejects_provably_wrong_arguments
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/schema_check/describes_required_fields_first_with_types_and_options
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/schema_check/accepts_well_formed_and_alias_arguments
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/check_capabilities
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn schema() -> Value`

# Called by

- [rejects_provably_wrong_arguments](../../../../../functions/src/llm/tools/schema_check/rejects_provably_wrong_arguments.md)
- [describes_required_fields_first_with_types_and_options](../../../../../functions/src/llm/tools/schema_check/describes_required_fields_first_with_types_and_options.md)
- [accepts_well_formed_and_alias_arguments](../../../../../functions/src/llm/tools/schema_check/accepts_well_formed_and_alias_arguments.md)
- [check_capabilities](../../../../../functions/src/memory/code_knowledge/check_capabilities.md)