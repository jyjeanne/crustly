---
type: Rust Method
title: new
resource: src/memory/model.rs#L143-L148
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/memory/model/MemoryCategory/fromstr/from_str
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/model/MemorySource/fromstr/from_str
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/model/MemoryStatus/fromstr/from_str
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn new(kind: &'static str, value: &str) -> Self`

# Called by

- [from_str](../../../../../functions/src/memory/model/MemoryCategory/fromstr/from_str.md)
- [from_str](../../../../../functions/src/memory/model/MemorySource/fromstr/from_str.md)
- [from_str](../../../../../functions/src/memory/model/MemoryStatus/fromstr/from_str.md)