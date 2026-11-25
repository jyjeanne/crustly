---
type: Rust Function
title: fenced_json_blocks
resource: src/llm/provider/tool_call_recovery.rs#L208-L226
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/tool_call_recovery/tool_call_from_content
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn fenced_json_blocks(content: &str) -> Vec<&str>`

# Calls

- [find](../../../../../functions/src/memory/MemoryStore/find.md)

# Called by

- [tool_call_from_content](../../../../../functions/src/llm/provider/tool_call_recovery/tool_call_from_content.md)