---
type: Rust Function
title: cmd_logs
resource: src/cli/mod.rs#L1231-L1393
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/len
  - functions/src/logging/get_log_path
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/logging/cleanup_old_logs
  called_by:
  - functions/src/cli/run
---

# Signature

`async fn cmd_logs(operation: LogCommands) -> Result<()>`

# Calls

- [len](../../../functions/src/config/secrets/SecretString/len.md)
- [get_log_path](../../../functions/src/logging/get_log_path.md)
- [is_empty](../../../functions/src/config/secrets/SecretString/is_empty.md)
- [cleanup_old_logs](../../../functions/src/logging/cleanup_old_logs.md)

# Called by

- [run](../../../functions/src/cli/run.md)