---
type: Rust Method
title: new
resource: src/llm/tools/registry.rs#L42-L47
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/cli/build_tool_registry
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/test_send_message_with_tool_execution
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/registry_of
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/estimated_overflow_tries_deeper_compaction_first
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/a_nearly_full_window_is_cut_deeper_before_sending
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/stopping_mid_round_answers_every_pending_tool_call
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/bad_tool_arguments_get_the_expected_arguments_back
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/agent_registry
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/a_refused_forget_deletes_nothing_and_the_prompt_shows_the_memory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/a_refused_global_save_stores_nothing
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/ToolRegistry/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/test_registry_creation
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/test_register_tool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/test_list_tools
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
  - target: functions/src/llm/tools/registry/register_mcp_server_with_nonexistent_command_fails_gracefully
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/get_resolves_a_known_alias_to_the_registered_canonical_tool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/has_tool_is_false_for_an_alias_whose_target_is_not_registered
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/an_exact_match_wins_over_an_alias_entry
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/resolve_name_maps_aliases_and_keeps_unknown_names
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/is_denied_follows_the_policy_through_aliases
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/is_denied_is_false_without_a_policy
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
  - target: functions/src/llm/tools/registry/is_trusted_evaluates_policy_against_the_canonical_name_not_the_alias
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/error_scenarios_test/create_error_agent
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/integration_test/create_test_agent
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new() -> Self`

# Called by

- [build_tool_registry](../../../../../../functions/src/cli/build_tool_registry.md)
- [new](../../../../../../functions/src/llm/agent/service/AgentService/new.md)
- [test_send_message_with_tool_execution](../../../../../../functions/src/llm/agent/service/test_send_message_with_tool_execution.md)
- [registry_of](../../../../../../functions/src/llm/agent/service/registry_of.md)
- [estimated_overflow_tries_deeper_compaction_first](../../../../../../functions/src/llm/agent/service/estimated_overflow_tries_deeper_compaction_first.md)
- [a_nearly_full_window_is_cut_deeper_before_sending](../../../../../../functions/src/llm/agent/service/a_nearly_full_window_is_cut_deeper_before_sending.md)
- [stopping_mid_round_answers_every_pending_tool_call](../../../../../../functions/src/llm/agent/service/stopping_mid_round_answers_every_pending_tool_call.md)
- [bad_tool_arguments_get_the_expected_arguments_back](../../../../../../functions/src/llm/agent/service/bad_tool_arguments_get_the_expected_arguments_back.md)
- [agent_registry](../../../../../../functions/src/llm/agent/service/agent_registry.md)
- [a_refused_forget_deletes_nothing_and_the_prompt_shows_the_memory](../../../../../../functions/src/llm/agent/service/a_refused_forget_deletes_nothing_and_the_prompt_shows_the_memory.md)
- [a_refused_global_save_stores_nothing](../../../../../../functions/src/llm/agent/service/a_refused_global_save_stores_nothing.md)
- [default](../../../../../../functions/src/llm/tools/registry/ToolRegistry/default/default.md)
- [test_registry_creation](../../../../../../functions/src/llm/tools/registry/test_registry_creation.md)
- [test_register_tool](../../../../../../functions/src/llm/tools/registry/test_register_tool.md)
- [test_list_tools](../../../../../../functions/src/llm/tools/registry/test_list_tools.md)
- [test_execute_tool](../../../../../../functions/src/llm/tools/registry/test_execute_tool.md)
- [test_execute_nonexistent_tool](../../../../../../functions/src/llm/tools/registry/test_execute_nonexistent_tool.md)
- [test_execute_requires_approval](../../../../../../functions/src/llm/tools/registry/test_execute_requires_approval.md)
- [test_execute_with_auto_approve](../../../../../../functions/src/llm/tools/registry/test_execute_with_auto_approve.md)
- [register_mcp_server_with_nonexistent_command_fails_gracefully](../../../../../../functions/src/llm/tools/registry/register_mcp_server_with_nonexistent_command_fails_gracefully.md)
- [get_resolves_a_known_alias_to_the_registered_canonical_tool](../../../../../../functions/src/llm/tools/registry/get_resolves_a_known_alias_to_the_registered_canonical_tool.md)
- [has_tool_is_false_for_an_alias_whose_target_is_not_registered](../../../../../../functions/src/llm/tools/registry/has_tool_is_false_for_an_alias_whose_target_is_not_registered.md)
- [an_exact_match_wins_over_an_alias_entry](../../../../../../functions/src/llm/tools/registry/an_exact_match_wins_over_an_alias_entry.md)
- [resolve_name_maps_aliases_and_keeps_unknown_names](../../../../../../functions/src/llm/tools/registry/resolve_name_maps_aliases_and_keeps_unknown_names.md)
- [is_denied_follows_the_policy_through_aliases](../../../../../../functions/src/llm/tools/registry/is_denied_follows_the_policy_through_aliases.md)
- [is_denied_is_false_without_a_policy](../../../../../../functions/src/llm/tools/registry/is_denied_is_false_without_a_policy.md)
- [execute_resolves_an_alias_name_to_the_registered_tool](../../../../../../functions/src/llm/tools/registry/execute_resolves_an_alias_name_to_the_registered_tool.md)
- [execute_reports_not_found_using_the_original_unresolved_name](../../../../../../functions/src/llm/tools/registry/execute_reports_not_found_using_the_original_unresolved_name.md)
- [execute_evaluates_policy_against_the_canonical_name_not_the_alias](../../../../../../functions/src/llm/tools/registry/execute_evaluates_policy_against_the_canonical_name_not_the_alias.md)
- [is_trusted_evaluates_policy_against_the_canonical_name_not_the_alias](../../../../../../functions/src/llm/tools/registry/is_trusted_evaluates_policy_against_the_canonical_name_not_the_alias.md)
- [create_error_agent](../../../../../../functions/tests/error_scenarios_test/create_error_agent.md)
- [create_test_agent](../../../../../../functions/tests/integration_test/create_test_agent.md)