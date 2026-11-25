---
type: Rust Method
title: exposes_mcp
resource: src/llm/agent/tool_profile.rs#L68-L84
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/tool_profile/ToolSelection/exposes
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn exposes_mcp(&self, name: &str) -> bool`

# Calls

- [find](../../../../../../functions/src/memory/MemoryStore/find.md)

# Called by

- [exposes](../../../../../../functions/src/llm/agent/tool_profile/ToolSelection/exposes.md)