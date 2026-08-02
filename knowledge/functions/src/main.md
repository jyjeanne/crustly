---
type: Rust Function
title: main
resource: src/main.rs#L6-L27
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/db/models/PlanTaskStatus/parse
  - functions/src/logging/setup_from_cli
  - functions/src/logging/cleanup_old_logs
---

# Signature

`async fn main() -> Result<()>`

# Calls

- [parse](../../functions/src/db/models/PlanTaskStatus/parse.md)
- [setup_from_cli](../../functions/src/logging/setup_from_cli.md)
- [cleanup_old_logs](../../functions/src/logging/cleanup_old_logs.md)