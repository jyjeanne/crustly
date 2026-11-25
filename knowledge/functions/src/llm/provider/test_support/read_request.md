---
type: Rust Function
title: read_request
resource: src/llm/provider/test_support.rs#L49-L76
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/test_support/held_response_server
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn read_request(socket: &mut tokio::net::TcpStream)`

# Called by

- [held_response_server](../../../../../functions/src/llm/provider/test_support/held_response_server.md)