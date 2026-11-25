---
type: Rust Method
title: with_allow_sub_agents
resource: src/llm/agent/service.rs#L825-L828
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/subagents/ChildTemplate/build
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn with_allow_sub_agents(mut self, allow: bool) -> Self`

# Called by

- [build](../../../../../../functions/src/llm/agent/service/subagents/ChildTemplate/build.md)