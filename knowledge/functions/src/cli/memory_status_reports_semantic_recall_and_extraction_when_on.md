---
type: Rust Function
title: memory_status_reports_semantic_recall_and_extraction_when_on
resource: src/cli/mod.rs#L4044-L4096
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/config/Config/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/cli/memory_status_lines
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn memory_status_reports_semantic_recall_and_extraction_when_on()`

# Calls

- [default](../../../functions/src/config/Config/default/default.md)
- [memory_status_lines](../../../functions/src/cli/memory_status_lines.md)
- [contains](../../../functions/src/memory/vector_index/VectorIndex/contains.md)