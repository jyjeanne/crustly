---
type: Rust Function
title: download_model_writes_the_file_and_reports_progress
resource: src/llm/provider/llama_cpp_models.rs#L1341-L1360
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
---

# Signature

`async fn download_model_writes_the_file_and_reports_progress()`

# Calls

- [mock_http_server](../../../../../functions/src/llm/provider/llama_cpp_models/mock_http_server.md)
- [download_model](../../../../../functions/src/llm/provider/llama_cpp_models/download_model.md)