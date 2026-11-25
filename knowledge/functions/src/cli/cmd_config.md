---
type: Rust Function
title: cmd_config
resource: src/cli/mod.rs#L1009-L1059
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/cli/run
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn cmd_config(config: &crate::config::Config, show_secrets: bool) -> Result<()>`

# Called by

- [run](../../../functions/src/cli/run.md)