---
type: Rust Method
title: with_request_limits
resource: src/llm/agent/service.rs#L624-L632
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/cli/configure_agent_loop
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn with_request_limits( mut self, max_wall_time: Option<Duration>, max_tokens: Option<u32>, ) -> Self`

# Called by

- [configure_agent_loop](../../../../../../functions/src/cli/configure_agent_loop.md)