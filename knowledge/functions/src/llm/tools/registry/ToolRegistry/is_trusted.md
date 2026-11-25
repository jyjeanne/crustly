---
type: Rust Method
title: is_trusted
resource: src/llm/tools/registry.rs#L67-L74
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/registry/ToolRegistry/canonical_name
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/tool_round/AgentService/execute_tool_round
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/registry/is_trusted_evaluates_policy_against_the_canonical_name_not_the_alias
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn is_trusted(&self, name: &str, input: &serde_json::Value) -> bool`

# Calls

- [canonical_name](../../../../../../functions/src/llm/tools/registry/ToolRegistry/canonical_name.md)

# Called by

- [execute_tool_round](../../../../../../functions/src/llm/agent/service/tool_round/AgentService/execute_tool_round.md)
- [is_trusted_evaluates_policy_against_the_canonical_name_not_the_alias](../../../../../../functions/src/llm/tools/registry/is_trusted_evaluates_policy_against_the_canonical_name_not_the_alias.md)