---
type: Rust Method
title: default
resource: src/config/mod.rs#L442-L447
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/config/default_max_concurrent_subagents
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/config/Config/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn default() -> Self`

# Calls

- [default_max_concurrent_subagents](../../../../../functions/src/config/default_max_concurrent_subagents.md)

# Called by

- [default](../../../../../functions/src/config/Config/default/default.md)