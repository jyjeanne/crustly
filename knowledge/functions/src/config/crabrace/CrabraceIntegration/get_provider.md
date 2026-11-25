---
type: Rust Method
title: get_provider
resource: src/config/crabrace.rs#L87-L90
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/config/crabrace/CrabraceIntegration/fetch_providers
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/config/crabrace/CrabraceIntegration/is_provider_available
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn get_provider(&self, provider_id: &str) -> Result<Option<Provider>>`

# Calls

- [fetch_providers](../../../../../functions/src/config/crabrace/CrabraceIntegration/fetch_providers.md)
- [find](../../../../../functions/src/memory/MemoryStore/find.md)

# Called by

- [is_provider_available](../../../../../functions/src/config/crabrace/CrabraceIntegration/is_provider_available.md)