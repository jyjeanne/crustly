---
type: Rust Method
title: default
resource: src/config/mod.rs#L171-L178
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/config/default_code_knowledge_enabled
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/default_code_knowledge_command
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/default_code_knowledge_timeout_ms
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/config/MemoryConfig/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/code_knowledge/detection_is_off_without_a_command_or_a_bundle
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn default() -> Self`

# Calls

- [default_code_knowledge_enabled](../../../../../functions/src/config/default_code_knowledge_enabled.md)
- [default_code_knowledge_command](../../../../../functions/src/config/default_code_knowledge_command.md)
- [default_code_knowledge_timeout_ms](../../../../../functions/src/config/default_code_knowledge_timeout_ms.md)

# Called by

- [default](../../../../../functions/src/config/MemoryConfig/default/default.md)
- [detection_is_off_without_a_command_or_a_bundle](../../../../../functions/src/memory/code_knowledge/detection_is_off_without_a_command_or_a_bundle.md)