---
type: Rust Module
title: mcp_contract_test
resource: tests/mcp_contract_test.rs#L1-L80
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/crustly-mcp-client-mcpclient
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [connect_to_nonexistent_server_returns_error](../../functions/tests/mcp_contract_test/connect_to_nonexistent_server_returns_error.md)
- [unhealthy_client_returns_graceful_error](../../functions/tests/mcp_contract_test/unhealthy_client_returns_graceful_error.md)
- [connect_times_out_gracefully](../../functions/tests/mcp_contract_test/connect_times_out_gracefully.md)

# Imports

- `crustly::mcp::client::MCPClient`

# Member of

- [crustly](../../packages/crustly.md)