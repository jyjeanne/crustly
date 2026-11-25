---
type: Rust Method
title: label
resource: src/llm/agent/tool_profile.rs#L123-L131
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/tool_round/AgentService/execute_tool_round
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/tool_profile/full_exposes_everything
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn label(&self) -> &'static str`

# Called by

- [execute_tool_round](../../../../../../functions/src/llm/agent/service/tool_round/AgentService/execute_tool_round.md)
- [full_exposes_everything](../../../../../../functions/src/llm/agent/tool_profile/full_exposes_everything.md)