---
type: Rust Method
title: max_request_tokens
resource: src/config/mod.rs#L420-L422
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/configure_agent_loop
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn max_request_tokens(&self) -> Option<u32>`

# Called by

- [configure_agent_loop](../../../../functions/src/cli/configure_agent_loop.md)