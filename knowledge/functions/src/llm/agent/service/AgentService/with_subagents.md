---
type: Rust Method
title: with_subagents
resource: src/llm/agent/service.rs#L612-L620
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/subagents/SubAgentManager/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/cli/configure_agent
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn with_subagents( mut self, max_concurrent: usize, profiles: std::collections::BTreeMap<String, super::profiles::AgentProfile>, ) -> Self`

# Calls

- [new](../../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/new.md)

# Called by

- [configure_agent](../../../../../../functions/src/cli/configure_agent.md)