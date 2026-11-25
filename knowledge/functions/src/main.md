---
type: Rust Function
title: main
resource: src/main.rs#L6-L27
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/logging/setup_from_cli
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/logging/cleanup_old_logs
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn main() -> Result<std::process::ExitCode>`

# Calls

- [setup_from_cli](../../functions/src/logging/setup_from_cli.md)
- [cleanup_old_logs](../../functions/src/logging/cleanup_old_logs.md)