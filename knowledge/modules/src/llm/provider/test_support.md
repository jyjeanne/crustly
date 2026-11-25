---
type: Rust Module
title: test_support
resource: src/llm/provider/test_support.rs#L1-L76
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/tokio-io-asyncreadext-asyncwriteext
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tokio-sync-oneshot
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [held_response_server](../../../../functions/src/llm/provider/test_support/held_response_server.md)
- [read_request](../../../../functions/src/llm/provider/test_support/read_request.md)

# Imports

- `tokio::io::{AsyncReadExt, AsyncWriteExt}`
- `tokio::sync::oneshot`

# Member of

- [crustly](../../../../packages/crustly.md)