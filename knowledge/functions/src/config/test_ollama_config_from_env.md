---
type: Rust Function
title: test_ollama_config_from_env
resource: src/config/mod.rs#L2279-L2314
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/config/Config/apply_env_overrides
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn test_ollama_config_from_env()`

# Calls

- [apply_env_overrides](../../../functions/src/config/Config/apply_env_overrides.md)
- [default](../../../functions/src/config/Config/default/default.md)