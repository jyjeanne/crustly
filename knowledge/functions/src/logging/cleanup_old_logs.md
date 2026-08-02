---
type: Rust Function
title: cleanup_old_logs
resource: src/logging.rs#L266-L296
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/cli/cmd_logs
  - functions/src/main
---

# Signature

`pub fn cleanup_old_logs(max_age_days: u64) -> Result<usize, Box<dyn std::error::Error>>`

# Called by

- [cmd_logs](../../../functions/src/cli/cmd_logs.md)
- [main](../../../functions/src/main.md)