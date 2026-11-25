---
type: Rust Function
title: download_model_rejects_a_checksum_mismatch_and_cleans_up
resource: src/llm/provider/llama_cpp_models.rs#L1363-L1384
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/llama_cpp_models/mock_http_server
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/download_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn download_model_rejects_a_checksum_mismatch_and_cleans_up()`

# Calls

- [mock_http_server](../../../../../functions/src/llm/provider/llama_cpp_models/mock_http_server.md)
- [download_model](../../../../../functions/src/llm/provider/llama_cpp_models/download_model.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)