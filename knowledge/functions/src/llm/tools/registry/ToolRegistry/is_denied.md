---
type: Rust Method
title: is_denied
resource: src/llm/tools/registry.rs#L82-L89
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
  - target: functions/src/llm/tools/registry/is_denied_follows_the_policy_through_aliases
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/registry/is_denied_is_false_without_a_policy
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn is_denied(&self, name: &str, input: &serde_json::Value) -> bool`

# Calls

- [canonical_name](../../../../../../functions/src/llm/tools/registry/ToolRegistry/canonical_name.md)

# Called by

- [execute_tool_round](../../../../../../functions/src/llm/agent/service/tool_round/AgentService/execute_tool_round.md)
- [is_denied_follows_the_policy_through_aliases](../../../../../../functions/src/llm/tools/registry/is_denied_follows_the_policy_through_aliases.md)
- [is_denied_is_false_without_a_policy](../../../../../../functions/src/llm/tools/registry/is_denied_is_false_without_a_policy.md)