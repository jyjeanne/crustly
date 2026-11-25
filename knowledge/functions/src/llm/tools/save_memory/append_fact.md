---
type: Rust Function
title: append_fact
resource: src/llm/tools/save_memory.rs#L186-L207
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/events/EventHandler/next
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/save_memory/SaveMemoryTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/save_memory/append_fact_adds_header_to_a_file_that_lacks_one
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn append_fact(existing: &str, fact: &str) -> (String, bool)`

# Calls

- [next](../../../../../functions/src/tui/events/EventHandler/next.md)
- [push](../../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)

# Called by

- [execute](../../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/tool/execute.md)
- [append_fact_adds_header_to_a_file_that_lacks_one](../../../../../functions/src/llm/tools/save_memory/append_fact_adds_header_to_a_file_that_lacks_one.md)