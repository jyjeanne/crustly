---
type: Rust Method
title: get
resource: src/llm/tools/registry.rs#L129-L131
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/registry/ToolRegistry/canonical_name
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/run/AgentRun/run
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/tool_round/AgentService/execute_tool_round
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/an_exact_match_wins_over_an_alias_entry
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/get_resolves_a_known_alias_to_the_registered_canonical_tool
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn get(&self, name: &str) -> Option<Arc<dyn Tool>>`

# Calls

- [canonical_name](../../../../../../functions/src/llm/tools/registry/ToolRegistry/canonical_name.md)

# Called by

- [run](../../../../../../functions/src/llm/agent/service/run/AgentRun/run.md)
- [execute_tool_round](../../../../../../functions/src/llm/agent/service/tool_round/AgentService/execute_tool_round.md)
- [an_exact_match_wins_over_an_alias_entry](../../../../../../functions/src/llm/tools/registry/an_exact_match_wins_over_an_alias_entry.md)
- [get_resolves_a_known_alias_to_the_registered_canonical_tool](../../../../../../functions/src/llm/tools/registry/get_resolves_a_known_alias_to_the_registered_canonical_tool.md)