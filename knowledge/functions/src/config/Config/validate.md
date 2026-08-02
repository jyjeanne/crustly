---
type: Rust Method
title: validate
resource: src/config/mod.rs#L1078-L1108
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/cli/load_config
---

# Signature

`pub fn validate(&self) -> Result<()>`

# Calls

- [is_empty](../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [load_config](../../../../functions/src/cli/load_config.md)