---
type: Rust Function
title: test_config_env_overrides
resource: src/config/mod.rs#L2225-L2245
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/config/Config/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/Config/apply_env_overrides
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_config_env_overrides()`

# Calls

- [default](../../../functions/src/config/Config/default/default.md)
- [apply_env_overrides](../../../functions/src/config/Config/apply_env_overrides.md)