---
type: Rust Method
title: local_config_path
resource: src/config/mod.rs#L1414-L1416
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/config/Config/load
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/test_local_config_path
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn local_config_path() -> PathBuf`

# Called by

- [load](../../../../functions/src/config/Config/load.md)
- [test_local_config_path](../../../../functions/src/config/test_local_config_path.md)