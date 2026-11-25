---
type: Rust Function
title: run_waits_for_the_answer_and_returns_it
resource: src/llm/tools/agent.rs#L394-L427
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/trait/ToolExecutionContext/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/trait/ToolExecutionContext/with_read_only_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/ToolExecutionContext/with_sub_agent_launcher
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/agent/AgentTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn run_waits_for_the_answer_and_returns_it()`

# Calls

- [new](../../../../../functions/src/llm/tools/trait/ToolExecutionContext/new.md)
- [with_read_only_mode](../../../../../functions/src/llm/tools/trait/ToolExecutionContext/with_read_only_mode.md)
- [with_sub_agent_launcher](../../../../../functions/src/llm/tools/trait/ToolExecutionContext/with_sub_agent_launcher.md)
- [execute](../../../../../functions/src/llm/tools/agent/AgentTool/tool/execute.md)