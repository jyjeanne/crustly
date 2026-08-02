---
type: Rust Module
title: client
resource: src/mcp/client.rs#L1-L602
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/anyhow-context-result
  - external/serde-deserialize-serialize
  - external/serde-json-value
  - external/tokio-process-command
  - external/tokio-io-asyncwriteext
  - external/tokio-io-asyncreadext
  - external/crate-llm-tools-tool-toolcapability-toolexecutioncontext-toolresult
  - external/async-trait-async-trait
  - external/std-sync-arc
  - external/tokio-sync-mutex
  - external/super
  member_of:
  - packages/crustly
---

# Contains

- [JsonRpcRequest](../../../classes/src/mcp/client/JsonRpcRequest.md)
- [JsonRpcResponse](../../../classes/src/mcp/client/JsonRpcResponse.md)
- [ResponseMatch](../../../classes/src/mcp/client/ResponseMatch.md)
- [match_response_line](../../../functions/src/mcp/client/match_response_line.md)
- [McpToolDef](../../../classes/src/mcp/client/McpToolDef.md)
- [MCPClient](../../../classes/src/mcp/client/MCPClient.md)
- [connect](../../../functions/src/mcp/client/MCPClient/connect.md)
- [discover_tools](../../../functions/src/mcp/client/MCPClient/discover_tools.md)
- [call_tool](../../../functions/src/mcp/client/MCPClient/call_tool.md)
- [is_healthy](../../../functions/src/mcp/client/MCPClient/is_healthy.md)
- [server_name](../../../functions/src/mcp/client/MCPClient/server_name.md)
- [send_request](../../../functions/src/mcp/client/MCPClient/send_request.md)
- [read_response_line](../../../functions/src/mcp/client/MCPClient/read_response_line.md)
- [McpTool](../../../classes/src/mcp/client/McpTool.md)
- [namespaced_tool_name](../../../functions/src/mcp/client/namespaced_tool_name.md)
- [new](../../../functions/src/mcp/client/McpTool/new.md)
- [name](../../../functions/src/mcp/client/McpTool/tool/name.md)
- [description](../../../functions/src/mcp/client/McpTool/tool/description.md)
- [input_schema](../../../functions/src/mcp/client/McpTool/tool/input_schema.md)
- [capabilities](../../../functions/src/mcp/client/McpTool/tool/capabilities.md)
- [requires_approval](../../../functions/src/mcp/client/McpTool/tool/requires_approval.md)
- [execute](../../../functions/src/mcp/client/McpTool/tool/execute.md)
- [matches_a_response_with_the_expected_id](../../../functions/src/mcp/client/matches_a_response_with_the_expected_id.md)
- [skips_a_response_for_a_different_request_id](../../../functions/src/mcp/client/skips_a_response_for_a_different_request_id.md)
- [skips_an_id_less_notification](../../../functions/src/mcp/client/skips_an_id_less_notification.md)
- [skips_an_unparseable_line](../../../functions/src/mcp/client/skips_an_unparseable_line.md)
- [surfaces_a_server_error_for_the_matching_id](../../../functions/src/mcp/client/surfaces_a_server_error_for_the_matching_id.md)
- [missing_result_defaults_to_null](../../../functions/src/mcp/client/missing_result_defaults_to_null.md)
- [send_request_skips_a_notification_and_matches_the_response_for_its_own_id](../../../functions/src/mcp/client/send_request_skips_a_notification_and_matches_the_response_for_its_own_id.md)
- [send_request_errors_when_the_server_process_is_gone](../../../functions/src/mcp/client/send_request_errors_when_the_server_process_is_gone.md)
- [mcp_tool_always_requires_approval_regardless_of_empty_capabilities](../../../functions/src/mcp/client/mcp_tool_always_requires_approval_regardless_of_empty_capabilities.md)
- [namespaced_tool_name_contains_no_colons](../../../functions/src/mcp/client/namespaced_tool_name_contains_no_colons.md)
- [namespaced_tool_name_uses_double_underscore_convention](../../../functions/src/mcp/client/namespaced_tool_name_uses_double_underscore_convention.md)
- [namespaced_tool_name_matches_provider_function_name_pattern](../../../functions/src/mcp/client/namespaced_tool_name_matches_provider_function_name_pattern.md)

# Imports

- `anyhow::{Context, Result}`
- `serde::{Deserialize, Serialize}`
- `serde_json::Value`
- `tokio::process::Command`
- `tokio::io::AsyncWriteExt`
- `tokio::io::AsyncReadExt`
- `crate::llm::tools::{Tool, ToolCapability, ToolExecutionContext, ToolResult}`
- `async_trait::async_trait`
- `std::sync::Arc`
- `tokio::sync::Mutex`
- `super::*`

# Member of

- [crustly](../../../packages/crustly.md)