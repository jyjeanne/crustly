---
type: Rust Function
title: run
resource: src/cli/mod.rs#L874-L956
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/cli/load_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/ProviderConfigs/override_default_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/llama_cpp_exit_code
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_chat
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_init
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_logs
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_keyring
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_autoplan
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn run() -> Result<std::process::ExitCode>`

# Calls

- [load_config](../../../functions/src/cli/load_config.md)
- [override_default_model](../../../functions/src/config/ProviderConfigs/override_default_model.md)
- [llama_cpp_exit_code](../../../functions/src/cli/llama_cpp_exit_code.md)
- [cmd_chat](../../../functions/src/cli/cmd_chat.md)
- [cmd_init](../../../functions/src/cli/cmd_init.md)
- [cmd_config](../../../functions/src/cli/cmd_config.md)
- [cmd_db](../../../functions/src/cli/cmd_db.md)
- [cmd_memory](../../../functions/src/cli/cmd_memory.md)
- [cmd_logs](../../../functions/src/cli/cmd_logs.md)
- [cmd_keyring](../../../functions/src/cli/cmd_keyring.md)
- [cmd_run](../../../functions/src/cli/cmd_run.md)
- [cmd_autoplan](../../../functions/src/cli/cmd_autoplan.md)