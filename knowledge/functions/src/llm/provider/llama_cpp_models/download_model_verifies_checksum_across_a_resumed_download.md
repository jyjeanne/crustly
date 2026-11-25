---
type: Rust Function
title: download_model_verifies_checksum_across_a_resumed_download
resource: src/llm/provider/llama_cpp_models.rs#L1526-L1550
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/llama_cpp_models/to_hex
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/write
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/mock_resumable_http_server
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/download_model
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn download_model_verifies_checksum_across_a_resumed_download()`

# Calls

- [to_hex](../../../../../functions/src/llm/provider/llama_cpp_models/to_hex.md)
- [write](../../../../../functions/src/memory/project/write.md)
- [mock_resumable_http_server](../../../../../functions/src/llm/provider/llama_cpp_models/mock_resumable_http_server.md)
- [download_model](../../../../../functions/src/llm/provider/llama_cpp_models/download_model.md)