---
type: Rust Function
title: llama_cpp_doctor_findings_reports_a_writable_models_dir_as_ok
resource: src/cli/mod.rs#L3704-L3717
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
  - target: functions/src/cli/llama_cpp_doctor_findings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/exists
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn llama_cpp_doctor_findings_reports_a_writable_models_dir_as_ok()`

# Calls

- [default](../../../functions/src/config/Config/default/default.md)
- [llama_cpp_doctor_findings](../../../functions/src/cli/llama_cpp_doctor_findings.md)
- [find](../../../functions/src/memory/MemoryStore/find.md)
- [contains](../../../functions/src/memory/vector_index/VectorIndex/contains.md)
- [exists](../../../functions/src/memory/code_knowledge/CodeKnowledge/exists.md)