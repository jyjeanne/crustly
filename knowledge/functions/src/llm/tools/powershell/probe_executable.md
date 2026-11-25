---
type: Rust Function
title: probe_executable
resource: src/llm/tools/powershell.rs#L31-L46
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/code_knowledge/CodeKnowledge/status
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn probe_executable(cmd: &str) -> bool`

# Calls

- [status](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/status.md)