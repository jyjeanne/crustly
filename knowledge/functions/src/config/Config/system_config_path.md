---
type: Rust Method
title: system_config_path
resource: src/config/mod.rs#L720-L722
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/config/Config/load
  - functions/src/config/test_system_config_path
---

# Signature

`fn system_config_path() -> Option<PathBuf>`

# Called by

- [load](../../../../functions/src/config/Config/load.md)
- [test_system_config_path](../../../../functions/src/config/test_system_config_path.md)