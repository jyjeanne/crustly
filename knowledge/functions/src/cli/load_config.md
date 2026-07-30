---
type: Rust Function
title: load_config
resource: src/cli/mod.rs#L387-L402
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/Config/load_from_path
  - functions/src/config/Config/validate
  called_by:
  - functions/src/cli/run
---

# Signature

`async fn load_config(config_path: Option<&str>) -> Result<crate::config::Config>`

# Calls

- [load_from_path](../../../functions/src/config/Config/load_from_path.md)
- [validate](../../../functions/src/config/Config/validate.md)

# Called by

- [run](../../../functions/src/cli/run.md)