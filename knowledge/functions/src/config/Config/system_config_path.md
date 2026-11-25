---
type: Rust Method
title: system_config_path
resource: src/config/mod.rs#L1480-L1482
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/config/Config/load
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/test_system_config_path
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn system_config_path() -> Option<PathBuf>`

# Called by

- [load](../../../../functions/src/config/Config/load.md)
- [test_system_config_path](../../../../functions/src/config/test_system_config_path.md)