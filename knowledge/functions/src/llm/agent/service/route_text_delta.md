---
type: Rust Function
title: route_text_delta
resource: src/llm/agent/service.rs#L329-L374
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/drain_stream_to_response
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn route_text_delta( input: &str, in_think: &mut bool, text_buf: &mut String, thinking_buf: &mut String, chunk_tx: Option<&mpsc::UnboundedSender<String>>, )`

# Called by

- [drain_stream_to_response](../../../../../functions/src/llm/agent/service/drain_stream_to_response.md)