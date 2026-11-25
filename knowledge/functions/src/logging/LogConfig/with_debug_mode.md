---
type: Rust Method
title: with_debug_mode
resource: src/logging.rs#L53-L59
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/logging/setup_from_cli
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/logging/test_log_config_with_debug
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn with_debug_mode(mut self, enabled: bool) -> Self`

# Called by

- [setup_from_cli](../../../../functions/src/logging/setup_from_cli.md)
- [test_log_config_with_debug](../../../../functions/src/logging/test_log_config_with_debug.md)