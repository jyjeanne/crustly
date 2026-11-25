---
type: Rust Module
title: schema_check
resource: src/llm/tools/schema_check.rs#L1-L199
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/serde-json-value
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-json-json
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [check_arguments](../../../../functions/src/llm/tools/schema_check/check_arguments.md)
- [describe_arguments](../../../../functions/src/llm/tools/schema_check/describe_arguments.md)
- [declared_types](../../../../functions/src/llm/tools/schema_check/declared_types.md)
- [kind_fits](../../../../functions/src/llm/tools/schema_check/kind_fits.md)
- [kind_of](../../../../functions/src/llm/tools/schema_check/kind_of.md)
- [schema](../../../../functions/src/llm/tools/schema_check/schema.md)
- [accepts_well_formed_and_alias_arguments](../../../../functions/src/llm/tools/schema_check/accepts_well_formed_and_alias_arguments.md)
- [rejects_provably_wrong_arguments](../../../../functions/src/llm/tools/schema_check/rejects_provably_wrong_arguments.md)
- [describes_required_fields_first_with_types_and_options](../../../../functions/src/llm/tools/schema_check/describes_required_fields_first_with_types_and_options.md)
- [describes_a_schema_without_properties](../../../../functions/src/llm/tools/schema_check/describes_a_schema_without_properties.md)

# Imports

- `serde_json::Value`
- `super::*`
- `serde_json::json`

# Member of

- [crustly](../../../../packages/crustly.md)