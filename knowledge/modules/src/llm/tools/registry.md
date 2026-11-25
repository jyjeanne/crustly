---
type: Rust Module
title: registry
resource: src/llm/tools/registry.rs#L1-L692
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/super-error-result-toolerror
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-r-trait-tool-toolexecutioncontext-toolresult
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-json-value
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-collections-hashmap
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-arc
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-tools-sandbox-policydecision
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-mcp-client-mcpclient-mcptool
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tokio-sync-mutex
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-tools-r-trait-toolcapability
    resolved_by: tree-sitter
    confidence: exact
  - target: external/async-trait-async-trait
    resolved_by: tree-sitter
    confidence: exact
  - target: external/uuid-uuid
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-tools-sandbox-denytoolrule
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-tools-sandbox-allowtoolrule
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [preview_input](../../../../functions/src/llm/tools/registry/preview_input.md)
- [ToolRegistry](../../../../classes/src/llm/tools/registry/ToolRegistry.md)
- [new](../../../../functions/src/llm/tools/registry/ToolRegistry/new.md)
- [set_policy](../../../../functions/src/llm/tools/registry/ToolRegistry/set_policy.md)
- [is_trusted](../../../../functions/src/llm/tools/registry/ToolRegistry/is_trusted.md)
- [is_denied](../../../../functions/src/llm/tools/registry/ToolRegistry/is_denied.md)
- [resolve_name](../../../../functions/src/llm/tools/registry/ToolRegistry/resolve_name.md)
- [register](../../../../functions/src/llm/tools/registry/ToolRegistry/register.md)
- [canonical_name](../../../../functions/src/llm/tools/registry/ToolRegistry/canonical_name.md)
- [get](../../../../functions/src/llm/tools/registry/ToolRegistry/get.md)
- [has_tool](../../../../functions/src/llm/tools/registry/ToolRegistry/has_tool.md)
- [list_tools](../../../../functions/src/llm/tools/registry/ToolRegistry/list_tools.md)
- [get_tool_definitions](../../../../functions/src/llm/tools/registry/ToolRegistry/get_tool_definitions.md)
- [execute](../../../../functions/src/llm/tools/registry/ToolRegistry/execute.md)
- [register_mcp_server](../../../../functions/src/llm/tools/registry/ToolRegistry/register_mcp_server.md)
- [count](../../../../functions/src/llm/tools/registry/ToolRegistry/count.md)
- [default](../../../../functions/src/llm/tools/registry/ToolRegistry/default/default.md)
- [preview_input_shows_the_command](../../../../functions/src/llm/tools/registry/preview_input_shows_the_command.md)
- [preview_input_truncates_a_large_payload](../../../../functions/src/llm/tools/registry/preview_input_truncates_a_large_payload.md)
- [preview_input_truncates_on_char_boundaries](../../../../functions/src/llm/tools/registry/preview_input_truncates_on_char_boundaries.md)
- [MockTool](../../../../classes/src/llm/tools/registry/MockTool.md)
- [name](../../../../functions/src/llm/tools/registry/MockTool/tool/name.md)
- [description](../../../../functions/src/llm/tools/registry/MockTool/tool/description.md)
- [input_schema](../../../../functions/src/llm/tools/registry/MockTool/tool/input_schema.md)
- [capabilities](../../../../functions/src/llm/tools/registry/MockTool/tool/capabilities.md)
- [requires_approval](../../../../functions/src/llm/tools/registry/MockTool/tool/requires_approval.md)
- [execute](../../../../functions/src/llm/tools/registry/MockTool/tool/execute.md)
- [test_registry_creation](../../../../functions/src/llm/tools/registry/test_registry_creation.md)
- [test_register_tool](../../../../functions/src/llm/tools/registry/test_register_tool.md)
- [test_list_tools](../../../../functions/src/llm/tools/registry/test_list_tools.md)
- [test_execute_tool](../../../../functions/src/llm/tools/registry/test_execute_tool.md)
- [test_execute_nonexistent_tool](../../../../functions/src/llm/tools/registry/test_execute_nonexistent_tool.md)
- [test_execute_requires_approval](../../../../functions/src/llm/tools/registry/test_execute_requires_approval.md)
- [test_execute_with_auto_approve](../../../../functions/src/llm/tools/registry/test_execute_with_auto_approve.md)
- [register_mcp_server_with_nonexistent_command_fails_gracefully](../../../../functions/src/llm/tools/registry/register_mcp_server_with_nonexistent_command_fails_gracefully.md)
- [get_resolves_a_known_alias_to_the_registered_canonical_tool](../../../../functions/src/llm/tools/registry/get_resolves_a_known_alias_to_the_registered_canonical_tool.md)
- [has_tool_is_false_for_an_alias_whose_target_is_not_registered](../../../../functions/src/llm/tools/registry/has_tool_is_false_for_an_alias_whose_target_is_not_registered.md)
- [an_exact_match_wins_over_an_alias_entry](../../../../functions/src/llm/tools/registry/an_exact_match_wins_over_an_alias_entry.md)
- [resolve_name_maps_aliases_and_keeps_unknown_names](../../../../functions/src/llm/tools/registry/resolve_name_maps_aliases_and_keeps_unknown_names.md)
- [is_denied_follows_the_policy_through_aliases](../../../../functions/src/llm/tools/registry/is_denied_follows_the_policy_through_aliases.md)
- [is_denied_is_false_without_a_policy](../../../../functions/src/llm/tools/registry/is_denied_is_false_without_a_policy.md)
- [execute_resolves_an_alias_name_to_the_registered_tool](../../../../functions/src/llm/tools/registry/execute_resolves_an_alias_name_to_the_registered_tool.md)
- [execute_reports_not_found_using_the_original_unresolved_name](../../../../functions/src/llm/tools/registry/execute_reports_not_found_using_the_original_unresolved_name.md)
- [execute_evaluates_policy_against_the_canonical_name_not_the_alias](../../../../functions/src/llm/tools/registry/execute_evaluates_policy_against_the_canonical_name_not_the_alias.md)
- [is_trusted_evaluates_policy_against_the_canonical_name_not_the_alias](../../../../functions/src/llm/tools/registry/is_trusted_evaluates_policy_against_the_canonical_name_not_the_alias.md)

# Imports

- `super::error::{Result, ToolError}`
- `super::r#trait::{Tool, ToolExecutionContext, ToolResult}`
- `serde_json::Value`
- `std::collections::HashMap`
- `std::sync::Arc`
- `crate::llm::tools::sandbox::PolicyDecision`
- `crate::mcp::client::{MCPClient, McpTool}`
- `tokio::sync::Mutex`
- `super::*`
- `crate::llm::tools::r#trait::ToolCapability`
- `async_trait::async_trait`
- `uuid::Uuid`
- `crate::llm::tools::sandbox::DenyToolRule`
- `crate::llm::tools::sandbox::AllowToolRule`

# Member of

- [crustly](../../../../packages/crustly.md)