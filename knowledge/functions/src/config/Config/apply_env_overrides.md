---
type: Rust Method
title: apply_env_overrides
resource: src/config/mod.rs#L1452-L1494
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/models/PlanTaskStatus/parse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/load_provider_api_keys
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/config/Config/load
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/load_from_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/test_config_env_overrides
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/test_provider_config_api_keys_from_env
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/test_ollama_config_from_env
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn apply_env_overrides(mut config: Self) -> Result<Self>`

# Calls

- [parse](../../../../functions/src/db/models/PlanTaskStatus/parse.md)
- [load_provider_api_keys](../../../../functions/src/config/Config/load_provider_api_keys.md)

# Called by

- [load](../../../../functions/src/config/Config/load.md)
- [load_from_path](../../../../functions/src/config/Config/load_from_path.md)
- [test_config_env_overrides](../../../../functions/src/config/test_config_env_overrides.md)
- [test_provider_config_api_keys_from_env](../../../../functions/src/config/test_provider_config_api_keys_from_env.md)
- [test_ollama_config_from_env](../../../../functions/src/config/test_ollama_config_from_env.md)