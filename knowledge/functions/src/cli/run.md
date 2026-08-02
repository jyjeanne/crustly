---
type: Rust Function
title: run
resource: src/cli/mod.rs#L353-L409
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/db/models/PlanTaskStatus/parse
  - functions/src/cli/load_config
  - functions/src/config/ProviderConfigs/override_default_model
  - functions/src/cli/cmd_chat
  - functions/src/cli/cmd_init
  - functions/src/cli/cmd_config
  - functions/src/cli/cmd_db
  - functions/src/cli/cmd_logs
  - functions/src/cli/cmd_keyring
  - functions/src/cli/cmd_run
  - functions/src/cli/cmd_autoplan
---

# Signature

`pub async fn run() -> Result<()>`

# Calls

- [parse](../../../functions/src/db/models/PlanTaskStatus/parse.md)
- [load_config](../../../functions/src/cli/load_config.md)
- [override_default_model](../../../functions/src/config/ProviderConfigs/override_default_model.md)
- [cmd_chat](../../../functions/src/cli/cmd_chat.md)
- [cmd_init](../../../functions/src/cli/cmd_init.md)
- [cmd_config](../../../functions/src/cli/cmd_config.md)
- [cmd_db](../../../functions/src/cli/cmd_db.md)
- [cmd_logs](../../../functions/src/cli/cmd_logs.md)
- [cmd_keyring](../../../functions/src/cli/cmd_keyring.md)
- [cmd_run](../../../functions/src/cli/cmd_run.md)
- [cmd_autoplan](../../../functions/src/cli/cmd_autoplan.md)