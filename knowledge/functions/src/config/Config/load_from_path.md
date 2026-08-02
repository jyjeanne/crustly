---
type: Rust Method
title: load_from_path
resource: src/config/mod.rs#L815-L834
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/Config/merge_from_file
  - functions/src/config/Config/apply_env_overrides
  called_by:
  - functions/src/cli/load_config
---

# Signature

`pub fn load_from_path<P: AsRef<Path>>(path: P) -> Result<Self>`

# Calls

- [merge_from_file](../../../../functions/src/config/Config/merge_from_file.md)
- [apply_env_overrides](../../../../functions/src/config/Config/apply_env_overrides.md)

# Called by

- [load_config](../../../../functions/src/cli/load_config.md)