---
type: Rust Method
title: register
resource: src/llm/tools/registry.rs#L77-L81
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/cli/build_tool_registry
  - functions/src/llm/agent/service/test_send_message_with_tool_execution
  - functions/src/llm/tools/registry/test_register_tool
  - functions/src/llm/tools/registry/test_list_tools
  - functions/src/llm/tools/registry/test_execute_tool
  - functions/src/llm/tools/registry/test_execute_requires_approval
  - functions/src/llm/tools/registry/test_execute_with_auto_approve
  - functions/src/llm/tools/registry/get_resolves_a_known_alias_to_the_registered_canonical_tool
  - functions/src/llm/tools/registry/an_exact_match_wins_over_an_alias_entry
  - functions/src/llm/tools/registry/execute_resolves_an_alias_name_to_the_registered_tool
  - functions/src/llm/tools/registry/execute_evaluates_policy_against_the_canonical_name_not_the_alias
  - functions/src/llm/tools/registry/is_trusted_evaluates_policy_against_the_canonical_name_not_the_alias
  - functions/tests/error_scenarios_test/create_error_agent
  - functions/tests/integration_test/create_test_agent
---

# Signature

`pub fn register(&mut self, tool: Arc<dyn Tool>)`

# Called by

- [build_tool_registry](../../../../../../functions/src/cli/build_tool_registry.md)
- [test_send_message_with_tool_execution](../../../../../../functions/src/llm/agent/service/test_send_message_with_tool_execution.md)
- [test_register_tool](../../../../../../functions/src/llm/tools/registry/test_register_tool.md)
- [test_list_tools](../../../../../../functions/src/llm/tools/registry/test_list_tools.md)
- [test_execute_tool](../../../../../../functions/src/llm/tools/registry/test_execute_tool.md)
- [test_execute_requires_approval](../../../../../../functions/src/llm/tools/registry/test_execute_requires_approval.md)
- [test_execute_with_auto_approve](../../../../../../functions/src/llm/tools/registry/test_execute_with_auto_approve.md)
- [get_resolves_a_known_alias_to_the_registered_canonical_tool](../../../../../../functions/src/llm/tools/registry/get_resolves_a_known_alias_to_the_registered_canonical_tool.md)
- [an_exact_match_wins_over_an_alias_entry](../../../../../../functions/src/llm/tools/registry/an_exact_match_wins_over_an_alias_entry.md)
- [execute_resolves_an_alias_name_to_the_registered_tool](../../../../../../functions/src/llm/tools/registry/execute_resolves_an_alias_name_to_the_registered_tool.md)
- [execute_evaluates_policy_against_the_canonical_name_not_the_alias](../../../../../../functions/src/llm/tools/registry/execute_evaluates_policy_against_the_canonical_name_not_the_alias.md)
- [is_trusted_evaluates_policy_against_the_canonical_name_not_the_alias](../../../../../../functions/src/llm/tools/registry/is_trusted_evaluates_policy_against_the_canonical_name_not_the_alias.md)
- [create_error_agent](../../../../../../functions/tests/error_scenarios_test/create_error_agent.md)
- [create_test_agent](../../../../../../functions/tests/integration_test/create_test_agent.md)