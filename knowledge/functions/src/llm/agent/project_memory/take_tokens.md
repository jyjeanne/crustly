---
type: Rust Function
title: take_tokens
resource: src/llm/agent/project_memory.rs#L86-L102
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/context/token_count
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/project_memory/load_project_notes
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn take_tokens(text: &str, max_tokens: u32) -> (String, bool)`

# Calls

- [token_count](../../../../../functions/src/llm/agent/context/token_count.md)

# Called by

- [load_project_notes](../../../../../functions/src/llm/agent/project_memory/load_project_notes.md)