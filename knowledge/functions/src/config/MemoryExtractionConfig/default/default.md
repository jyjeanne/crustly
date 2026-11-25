---
type: Rust Method
title: default
resource: src/config/mod.rs#L286-L296
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/config/default_extraction_provider
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/default_extraction_every
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/default_extraction_max
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/default_extraction_min_importance
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/config/MemoryConfig/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_extraction_test/enabled_config
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn default() -> Self`

# Calls

- [default_extraction_provider](../../../../../functions/src/config/default_extraction_provider.md)
- [default_extraction_every](../../../../../functions/src/config/default_extraction_every.md)
- [default_extraction_max](../../../../../functions/src/config/default_extraction_max.md)
- [default_extraction_min_importance](../../../../../functions/src/config/default_extraction_min_importance.md)

# Called by

- [default](../../../../../functions/src/config/MemoryConfig/default/default.md)
- [enabled_config](../../../../../functions/tests/memory_extraction_test/enabled_config.md)