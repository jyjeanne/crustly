---
type: Rust Method
title: save
resource: src/config/mod.rs#L1760-L1775
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/error/ErrorInfo/with_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/write
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_init
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/test_config_save_and_load
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn save(&self, path: &Path) -> Result<()>`

# Calls

- [context](../../../../functions/src/services/ServiceManager/context.md)
- [with_context](../../../../functions/src/tui/error/ErrorInfo/with_context.md)
- [write](../../../../functions/src/memory/project/write.md)

# Called by

- [cmd_init](../../../../functions/src/cli/cmd_init.md)
- [test_config_save_and_load](../../../../functions/src/config/test_config_save_and_load.md)