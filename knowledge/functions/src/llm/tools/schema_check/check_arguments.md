---
type: Rust Function
title: check_arguments
resource: src/llm/tools/schema_check.rs#L21-L50
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/schema_check/kind_fits
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/schema_check/kind_of
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/tool_round/AgentService/execute_tool_round
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/schema_check/rejects_provably_wrong_arguments
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/schema_check/accepts_well_formed_and_alias_arguments
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn check_arguments(input: &Value, schema: &Value) -> Result<(), String>`

# Calls

- [kind_fits](../../../../../functions/src/llm/tools/schema_check/kind_fits.md)
- [kind_of](../../../../../functions/src/llm/tools/schema_check/kind_of.md)

# Called by

- [execute_tool_round](../../../../../functions/src/llm/agent/service/tool_round/AgentService/execute_tool_round.md)
- [rejects_provably_wrong_arguments](../../../../../functions/src/llm/tools/schema_check/rejects_provably_wrong_arguments.md)
- [accepts_well_formed_and_alias_arguments](../../../../../functions/src/llm/tools/schema_check/accepts_well_formed_and_alias_arguments.md)