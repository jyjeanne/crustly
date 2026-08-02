---
type: Rust Function
title: cmd_autoplan
resource: src/cli/mod.rs#L1368-L1391
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/cli/cmd_run
  called_by:
  - functions/src/cli/run
---

# Signature

`async fn cmd_autoplan( config: &crate::config::Config, goal: String, max_iterations: u32, ) -> Result<()>`

# Calls

- [cmd_run](../../../functions/src/cli/cmd_run.md)

# Called by

- [run](../../../functions/src/cli/run.md)