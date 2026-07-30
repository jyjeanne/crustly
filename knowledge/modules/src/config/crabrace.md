---
type: Rust Module
title: crabrace
resource: src/config/crabrace.rs#L1-L144
generated:
  by: okf-rs/0.2.0
relationships:
  imports:
  - external/anyhow-context-result
  - external/crabrace-crabraceclient-provider
  - external/serde-deserialize-serialize
  - external/super
  member_of:
  - packages/crustly
---

# Contains

- [CrabraceConfig](../../../classes/src/config/crabrace/CrabraceConfig.md)
- [default_enabled](../../../functions/src/config/crabrace/default_enabled.md)
- [default_base_url](../../../functions/src/config/crabrace/default_base_url.md)
- [default_auto_update](../../../functions/src/config/crabrace/default_auto_update.md)
- [default_update_interval](../../../functions/src/config/crabrace/default_update_interval.md)
- [default](../../../functions/src/config/crabrace/CrabraceConfig/default/default.md)
- [CrabraceIntegration](../../../classes/src/config/crabrace/CrabraceIntegration.md)
- [new](../../../functions/src/config/crabrace/CrabraceIntegration/new.md)
- [fetch_providers](../../../functions/src/config/crabrace/CrabraceIntegration/fetch_providers.md)
- [health_check](../../../functions/src/config/crabrace/CrabraceIntegration/health_check.md)
- [get_provider](../../../functions/src/config/crabrace/CrabraceIntegration/get_provider.md)
- [get_all_model_ids](../../../functions/src/config/crabrace/CrabraceIntegration/get_all_model_ids.md)
- [is_provider_available](../../../functions/src/config/crabrace/CrabraceIntegration/is_provider_available.md)
- [config](../../../functions/src/config/crabrace/CrabraceIntegration/config.md)
- [test_default_config](../../../functions/src/config/crabrace/test_default_config.md)
- [test_create_integration](../../../functions/src/config/crabrace/test_create_integration.md)
- [test_health_check](../../../functions/src/config/crabrace/test_health_check.md)

# Imports

- `anyhow::{Context, Result}`
- `crabrace::{CrabraceClient, Provider}`
- `serde::{Deserialize, Serialize}`
- `super::*`

# Member of

- [crustly](../../../packages/crustly.md)