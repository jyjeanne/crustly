---
type: Rust Method
title: apply_env_overrides
resource: src/config/mod.rs#L771-L813
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/db/models/PlanTaskStatus/parse
  - functions/src/config/Config/load_provider_api_keys
  called_by:
  - functions/src/config/Config/load
  - functions/src/config/Config/load_from_path
  - functions/src/config/test_config_env_overrides
  - functions/src/config/test_provider_config_api_keys_from_env
  - functions/src/config/test_ollama_config_from_env
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