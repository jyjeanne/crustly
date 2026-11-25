---
type: Rust Function
title: describe_arguments
resource: src/llm/tools/schema_check.rs#L54-L102
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/schema_check/declared_types
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/tool_round/completed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/tool_round/AgentService/execute_tool_round
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/schema_check/describes_required_fields_first_with_types_and_options
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/schema_check/describes_a_schema_without_properties
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn describe_arguments(schema: &Value) -> String`

# Calls

- [declared_types](../../../../../functions/src/llm/tools/schema_check/declared_types.md)

# Called by

- [completed](../../../../../functions/src/llm/agent/service/tool_round/completed.md)
- [execute_tool_round](../../../../../functions/src/llm/agent/service/tool_round/AgentService/execute_tool_round.md)
- [describes_required_fields_first_with_types_and_options](../../../../../functions/src/llm/tools/schema_check/describes_required_fields_first_with_types_and_options.md)
- [describes_a_schema_without_properties](../../../../../functions/src/llm/tools/schema_check/describes_a_schema_without_properties.md)