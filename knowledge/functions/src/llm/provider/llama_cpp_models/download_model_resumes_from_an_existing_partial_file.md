---
type: Rust Function
title: download_model_resumes_from_an_existing_partial_file
resource: src/llm/provider/llama_cpp_models.rs#L1495-L1523
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
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

`async fn download_model_resumes_from_an_existing_partial_file()`

# Calls

- [write](../../../../../functions/src/memory/project/write.md)
- [mock_resumable_http_server](../../../../../functions/src/llm/provider/llama_cpp_models/mock_resumable_http_server.md)
- [download_model](../../../../../functions/src/llm/provider/llama_cpp_models/download_model.md)