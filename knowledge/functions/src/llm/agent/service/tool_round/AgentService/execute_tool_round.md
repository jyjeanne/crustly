---
type: Rust Method
title: execute_tool_round
resource: src/llm/agent/service/tool_round.rs#L119-L385
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/tool_profile/ToolSelection/exposes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/tool_round/ToolRound/refuse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/registry/ToolRegistry/get
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/schema_check/check_arguments
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/is_parallelizable
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/cache/CacheKey/from_tool
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/registry/ToolRegistry/is_denied
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/cache/ToolResultCache/get
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/tool_round/completed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/ToolResult/success
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/ToolRegistry/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/cache/ToolResultCache/insert_for_tool
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/EventHandler/next
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/plan_completion_rejection
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/registry/ToolRegistry/is_trusted
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/middleware/has_mutating_capability
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/cache/ToolResultCache/invalidate_matching
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/tool_profile/ToolSelection/label
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/schema_check/describe_arguments
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/run/AgentRun/run
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) async fn execute_tool_round( &self, tool_uses: Vec<ToolUse>, selection: &ToolSelection, tools: &RequestTools, tool_context: &ToolExecutionContext, mutating_evidence: &mut usize, round: usize, cancel: &CancellationToken, ) -> ToolRound`

# Calls

- [exposes](../../../../../../../functions/src/llm/agent/tool_profile/ToolSelection/exposes.md)
- [refuse](../../../../../../../functions/src/llm/agent/service/tool_round/ToolRound/refuse.md)
- [get](../../../../../../../functions/src/llm/tools/registry/ToolRegistry/get.md)
- [check_arguments](../../../../../../../functions/src/llm/tools/schema_check/check_arguments.md)
- [is_parallelizable](../../../../../../../functions/src/llm/agent/service/is_parallelizable.md)
- [from_tool](../../../../../../../functions/src/llm/tools/cache/CacheKey/from_tool.md)
- [is_denied](../../../../../../../functions/src/llm/tools/registry/ToolRegistry/is_denied.md)
- [get](../../../../../../../functions/src/llm/tools/cache/ToolResultCache/get.md)
- [completed](../../../../../../../functions/src/llm/agent/service/tool_round/completed.md)
- [success](../../../../../../../functions/src/llm/tools/trait/ToolResult/success.md)
- [execute](../../../../../../../functions/src/llm/tools/registry/ToolRegistry/execute.md)
- [insert_for_tool](../../../../../../../functions/src/llm/tools/cache/ToolResultCache/insert_for_tool.md)
- [push](../../../../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)
- [next](../../../../../../../functions/src/tui/events/EventHandler/next.md)
- [plan_completion_rejection](../../../../../../../functions/src/llm/agent/service/plan_completion_rejection.md)
- [is_trusted](../../../../../../../functions/src/llm/tools/registry/ToolRegistry/is_trusted.md)
- [has_mutating_capability](../../../../../../../functions/src/llm/agent/middleware/has_mutating_capability.md)
- [invalidate_matching](../../../../../../../functions/src/llm/tools/cache/ToolResultCache/invalidate_matching.md)
- [contains](../../../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
- [label](../../../../../../../functions/src/llm/agent/tool_profile/ToolSelection/label.md)
- [describe_arguments](../../../../../../../functions/src/llm/tools/schema_check/describe_arguments.md)

# Called by

- [run](../../../../../../../functions/src/llm/agent/service/run/AgentRun/run.md)