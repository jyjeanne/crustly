---
type: Rust Function
title: without_a_launcher_the_tool_says_so
resource: src/llm/tools/agent.rs#L430-L440
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/trait/ToolExecutionContext/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/agent/AgentTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn without_a_launcher_the_tool_says_so()`

# Calls

- [new](../../../../../functions/src/llm/tools/trait/ToolExecutionContext/new.md)
- [execute](../../../../../functions/src/llm/tools/agent/AgentTool/tool/execute.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)