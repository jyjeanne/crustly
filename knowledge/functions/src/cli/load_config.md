---
type: Rust Function
title: load_config
resource: src/cli/mod.rs#L915-L930
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/config/Config/load_from_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/load
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/Config/validate
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/run
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn load_config(config_path: Option<&str>) -> Result<crate::config::Config>`

# Calls

- [load_from_path](../../../functions/src/config/Config/load_from_path.md)
- [load](../../../functions/src/config/Config/load.md)
- [validate](../../../functions/src/config/Config/validate.md)

# Called by

- [run](../../../functions/src/cli/run.md)