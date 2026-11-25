---
type: Rust Method
title: max_wall_time
resource: src/config/mod.rs#L414-L417
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/configure_agent_loop
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn max_wall_time(&self) -> Option<std::time::Duration>`

# Called by

- [configure_agent_loop](../../../../functions/src/cli/configure_agent_loop.md)