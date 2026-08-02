---
type: Rust Function
title: download_model_writes_the_file_and_reports_progress
resource: src/llm/provider/llama_cpp_models.rs#L360-L379
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/llama_cpp_models/mock_http_server
  - functions/src/llm/provider/llama_cpp_models/download_model
---

# Signature

`async fn download_model_writes_the_file_and_reports_progress()`

# Calls

- [mock_http_server](../../../../../functions/src/llm/provider/llama_cpp_models/mock_http_server.md)
- [download_model](../../../../../functions/src/llm/provider/llama_cpp_models/download_model.md)