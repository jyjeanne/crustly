---
type: Rust Function
title: cmd_autoplan
resource: src/cli/mod.rs#L2912-L2935
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/cli/cmd_run
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/run
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn cmd_autoplan( config: &crate::config::Config, goal: String, max_iterations: u32, ) -> Result<()>`

# Calls

- [cmd_run](../../../functions/src/cli/cmd_run.md)

# Called by

- [run](../../../functions/src/cli/run.md)