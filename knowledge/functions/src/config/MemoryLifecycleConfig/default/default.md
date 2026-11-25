---
type: Rust Method
title: default
resource: src/config/mod.rs#L2417-L2421
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/config/default_episode_retention_days
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/config/MemoryConfig/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn default() -> Self`

# Calls

- [default_episode_retention_days](../../../../../functions/src/config/default_episode_retention_days.md)

# Called by

- [default](../../../../../functions/src/config/MemoryConfig/default/default.md)