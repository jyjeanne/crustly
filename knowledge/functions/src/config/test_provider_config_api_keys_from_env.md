---
type: Rust Function
title: test_provider_config_api_keys_from_env
resource: src/config/mod.rs#L2138-L2166
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
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

`fn test_provider_config_api_keys_from_env()`

# Calls

- [default](../../../functions/src/config/Config/default/default.md)
- [apply_env_overrides](../../../functions/src/config/Config/apply_env_overrides.md)