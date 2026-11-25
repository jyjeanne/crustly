---
type: Rust Method
title: is_provider_available
resource: src/config/crabrace.rs#L104-L106
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/config/crabrace/CrabraceIntegration/get_provider
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn is_provider_available(&self, provider_id: &str) -> Result<bool>`

# Calls

- [get_provider](../../../../../functions/src/config/crabrace/CrabraceIntegration/get_provider.md)