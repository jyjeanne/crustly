---
type: Rust Method
title: make_room
resource: src/llm/agent/service.rs#L943-L1007
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/AgentService/measure_budget
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/is_tight
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/compaction/elide_tool_results
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/compaction/compact_keeping
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/available
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/fixed_overhead
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/make_room_elides_old_tool_output_when_the_budget_is_tight
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/run/AgentRun/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/run/AgentRun/compact_deeper
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/run/AgentRun/finish
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn make_room( &self, context: &mut AgentContext, tools: &RequestTools, level: usize, ) -> bool`

# Calls

- [measure_budget](../../../../../../functions/src/llm/agent/service/AgentService/measure_budget.md)
- [is_tight](../../../../../../functions/src/llm/agent/budget/ContextBudget/is_tight.md)
- [elide_tool_results](../../../../../../functions/src/llm/agent/compaction/elide_tool_results.md)
- [compact_keeping](../../../../../../functions/src/llm/agent/compaction/compact_keeping.md)
- [available](../../../../../../functions/src/llm/agent/budget/ContextBudget/available.md)
- [fixed_overhead](../../../../../../functions/src/llm/agent/budget/ContextBudget/fixed_overhead.md)

# Called by

- [make_room_elides_old_tool_output_when_the_budget_is_tight](../../../../../../functions/src/llm/agent/service/make_room_elides_old_tool_output_when_the_budget_is_tight.md)
- [run](../../../../../../functions/src/llm/agent/service/run/AgentRun/run.md)
- [compact_deeper](../../../../../../functions/src/llm/agent/service/run/AgentRun/compact_deeper.md)
- [finish](../../../../../../functions/src/llm/agent/service/run/AgentRun/finish.md)