---
type: Rust Method
title: health_check
resource: src/config/crabrace.rs#L79-L84
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/config/crabrace/test_health_check
  - functions/src/config/update/ProviderUpdater/update
---

# Signature

`pub async fn health_check(&self) -> Result<bool>`

# Called by

- [test_health_check](../../../../../functions/src/config/crabrace/test_health_check.md)
- [update](../../../../../functions/src/config/update/ProviderUpdater/update.md)