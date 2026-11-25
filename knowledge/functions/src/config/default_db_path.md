---
type: Rust Function
title: default_db_path
resource: src/config/mod.rs#L1331-L1336
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/config/DatabaseConfig/default/default
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/default/default
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn default_db_path() -> PathBuf`

# Called by

- [default](../../../functions/src/config/DatabaseConfig/default/default.md)
- [default](../../../functions/src/config/Config/default/default.md)