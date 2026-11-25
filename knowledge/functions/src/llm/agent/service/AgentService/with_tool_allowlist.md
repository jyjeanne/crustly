---
type: Rust Method
title: with_tool_allowlist
resource: src/llm/agent/service.rs#L595-L598
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/subagents/ChildTemplate/build
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn with_tool_allowlist(mut self, tools: Vec<String>) -> Self`

# Called by

- [build](../../../../../../functions/src/llm/agent/service/subagents/ChildTemplate/build.md)