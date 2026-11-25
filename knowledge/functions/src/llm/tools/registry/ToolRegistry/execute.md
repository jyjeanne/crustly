---
type: Rust Method
title: execute
resource: src/llm/tools/registry.rs#L164-L245
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/registry/ToolRegistry/canonical_name
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/registry/preview_input
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/tool_round/AgentService/execute_tool_round
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/test_execute_tool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/test_execute_nonexistent_tool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/test_execute_requires_approval
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/test_execute_with_auto_approve
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/execute_resolves_an_alias_name_to_the_registered_tool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/execute_reports_not_found_using_the_original_unresolved_name
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/execute_evaluates_policy_against_the_canonical_name_not_the_alias
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn execute( &self, name: &str, input: Value, context: &ToolExecutionContext, ) -> Result<ToolResult>`

# Calls

- [canonical_name](../../../../../../functions/src/llm/tools/registry/ToolRegistry/canonical_name.md)
- [preview_input](../../../../../../functions/src/llm/tools/registry/preview_input.md)

# Called by

- [execute_tool_round](../../../../../../functions/src/llm/agent/service/tool_round/AgentService/execute_tool_round.md)
- [test_execute_tool](../../../../../../functions/src/llm/tools/registry/test_execute_tool.md)
- [test_execute_nonexistent_tool](../../../../../../functions/src/llm/tools/registry/test_execute_nonexistent_tool.md)
- [test_execute_requires_approval](../../../../../../functions/src/llm/tools/registry/test_execute_requires_approval.md)
- [test_execute_with_auto_approve](../../../../../../functions/src/llm/tools/registry/test_execute_with_auto_approve.md)
- [execute_resolves_an_alias_name_to_the_registered_tool](../../../../../../functions/src/llm/tools/registry/execute_resolves_an_alias_name_to_the_registered_tool.md)
- [execute_reports_not_found_using_the_original_unresolved_name](../../../../../../functions/src/llm/tools/registry/execute_reports_not_found_using_the_original_unresolved_name.md)
- [execute_evaluates_policy_against_the_canonical_name_not_the_alias](../../../../../../functions/src/llm/tools/registry/execute_evaluates_policy_against_the_canonical_name_not_the_alias.md)