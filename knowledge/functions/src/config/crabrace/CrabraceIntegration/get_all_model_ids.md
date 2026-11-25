---
type: Rust Method
title: get_all_model_ids
resource: src/config/crabrace.rs#L93-L101
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/config/crabrace/CrabraceIntegration/fetch_providers
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn get_all_model_ids(&self) -> Result<Vec<String>>`

# Calls

- [fetch_providers](../../../../../functions/src/config/crabrace/CrabraceIntegration/fetch_providers.md)