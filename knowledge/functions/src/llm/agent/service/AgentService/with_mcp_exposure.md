---
type: Rust Method
title: with_mcp_exposure
resource: src/llm/agent/service.rs#L713-L716
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/cli/configure_agent_loop
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/subagents/ChildTemplate/build
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn with_mcp_exposure(mut self, exposure: Vec<(String, crate::config::McpExpose)>) -> Self`

# Called by

- [configure_agent_loop](../../../../../../functions/src/cli/configure_agent_loop.md)
- [build](../../../../../../functions/src/llm/agent/service/subagents/ChildTemplate/build.md)