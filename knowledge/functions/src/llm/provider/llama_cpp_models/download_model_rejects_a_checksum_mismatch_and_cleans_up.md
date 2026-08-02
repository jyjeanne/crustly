---
type: Rust Function
title: download_model_rejects_a_checksum_mismatch_and_cleans_up
resource: src/llm/provider/llama_cpp_models.rs#L382-L403
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/llama_cpp_models/mock_http_server
  - functions/src/llm/provider/llama_cpp_models/download_model
---

# Signature

`async fn download_model_rejects_a_checksum_mismatch_and_cleans_up()`

# Calls

- [mock_http_server](../../../../../functions/src/llm/provider/llama_cpp_models/mock_http_server.md)
- [download_model](../../../../../functions/src/llm/provider/llama_cpp_models/download_model.md)