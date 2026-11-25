---
type: Rust Function
title: default_code_knowledge_timeout_ms
resource: src/config/mod.rs#L187-L189
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/config/MemoryCodeKnowledgeConfig/default/default
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/MemoryConfig/sanitized
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn default_code_knowledge_timeout_ms() -> u64`

# Called by

- [default](../../../functions/src/config/MemoryCodeKnowledgeConfig/default/default.md)
- [sanitized](../../../functions/src/config/MemoryConfig/sanitized.md)