---
type: Rust Method
title: load_provider_api_keys
resource: src/config/mod.rs#L816-L958
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/db/models/PlanTaskStatus/parse
  called_by:
  - functions/src/config/Config/apply_env_overrides
---

# Signature

`fn load_provider_api_keys(config: &mut Self) -> Result<()>`

# Calls

- [parse](../../../../functions/src/db/models/PlanTaskStatus/parse.md)

# Called by

- [apply_env_overrides](../../../../functions/src/config/Config/apply_env_overrides.md)