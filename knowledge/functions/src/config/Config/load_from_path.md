---
type: Rust Method
title: load_from_path
resource: src/config/mod.rs#L1463-L1477
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/config/Config/from_layers
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/apply_env_overrides
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/load_config
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn load_from_path<P: AsRef<Path>>(path: P) -> Result<Self>`

# Calls

- [from_layers](../../../../functions/src/config/Config/from_layers.md)
- [apply_env_overrides](../../../../functions/src/config/Config/apply_env_overrides.md)

# Called by

- [load_config](../../../../functions/src/cli/load_config.md)