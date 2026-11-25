---
type: Rust Method
title: validate
resource: src/config/mod.rs#L1642-L1672
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/load_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/test_config_validation
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/test_config_validation_invalid_log_level
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/test_config_validation_empty_crabrace_url
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn validate(&self) -> Result<()>`

# Calls

- [contains](../../../../functions/src/memory/vector_index/VectorIndex/contains.md)

# Called by

- [load_config](../../../../functions/src/cli/load_config.md)
- [test_config_validation](../../../../functions/src/config/test_config_validation.md)
- [test_config_validation_invalid_log_level](../../../../functions/src/config/test_config_validation_invalid_log_level.md)
- [test_config_validation_empty_crabrace_url](../../../../functions/src/config/test_config_validation_empty_crabrace_url.md)