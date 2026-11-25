---
type: Rust Module
title: update
resource: src/config/update.rs#L1-L277
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/anyhow-context-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crabrace-provider
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-time-duration-systemtime
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tokio-time
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tracing-debug-info-warn
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-crabrace-crabraceintegration
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-config-providerconfig
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-config-crabrace-crabraceconfig
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [ProviderUpdater](../../../classes/src/config/update/ProviderUpdater.md)
- [new](../../../functions/src/config/update/ProviderUpdater/new.md)
- [should_update](../../../functions/src/config/update/ProviderUpdater/should_update.md)
- [update](../../../functions/src/config/update/ProviderUpdater/update.md)
- [update_provider_config](../../../functions/src/config/update/ProviderUpdater/update_provider_config.md)
- [start_auto_update_loop](../../../functions/src/config/update/ProviderUpdater/start_auto_update_loop.md)
- [update_once](../../../functions/src/config/update/ProviderUpdater/update_once.md)
- [UpdateResult](../../../classes/src/config/update/UpdateResult.md)
- [success](../../../functions/src/config/update/UpdateResult/success.md)
- [failure](../../../functions/src/config/update/UpdateResult/failure.md)
- [test_should_update_when_disabled](../../../functions/src/config/update/test_should_update_when_disabled.md)
- [test_should_update_when_never_updated](../../../functions/src/config/update/test_should_update_when_never_updated.md)
- [test_update_result_success](../../../functions/src/config/update/test_update_result_success.md)
- [test_update_result_failure](../../../functions/src/config/update/test_update_result_failure.md)

# Imports

- `anyhow::{Context, Result}`
- `crabrace::Provider`
- `std::time::{Duration, SystemTime}`
- `tokio::time`
- `tracing::{debug, info, warn}`
- `super::crabrace::CrabraceIntegration`
- `super::{Config, ProviderConfig}`
- `super::*`
- `crate::config::crabrace::CrabraceConfig`

# Member of

- [crustly](../../../packages/crustly.md)