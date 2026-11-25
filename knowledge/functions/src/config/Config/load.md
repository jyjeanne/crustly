---
type: Rust Method
title: load
resource: src/config/mod.rs#L1337-L1370
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/config/Config/system_config_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/project_config_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/local_config_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/from_layers
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/apply_env_overrides
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/load_config
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn load() -> Result<Self>`

# Calls

- [system_config_path](../../../../functions/src/config/Config/system_config_path.md)
- [project_config_path](../../../../functions/src/config/Config/project_config_path.md)
- [local_config_path](../../../../functions/src/config/Config/local_config_path.md)
- [from_layers](../../../../functions/src/config/Config/from_layers.md)
- [apply_env_overrides](../../../../functions/src/config/Config/apply_env_overrides.md)

# Called by

- [load_config](../../../../functions/src/cli/load_config.md)