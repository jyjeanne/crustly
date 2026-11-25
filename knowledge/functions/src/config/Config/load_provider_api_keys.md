---
type: Rust Method
title: load_provider_api_keys
resource: src/config/mod.rs#L1582-L1724
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/models/PlanTaskStatus/parse
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/config/Config/apply_env_overrides
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn load_provider_api_keys(config: &mut Self) -> Result<()>`

# Calls

- [parse](../../../../functions/src/db/models/PlanTaskStatus/parse.md)

# Called by

- [apply_env_overrides](../../../../functions/src/config/Config/apply_env_overrides.md)