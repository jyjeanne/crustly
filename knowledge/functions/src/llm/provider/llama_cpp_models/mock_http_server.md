---
type: Rust Function
title: mock_http_server
resource: src/llm/provider/llama_cpp_models.rs#L409-L446
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/provider/llama_cpp_models/download_model_writes_the_file_and_reports_progress
  - functions/src/llm/provider/llama_cpp_models/download_model_rejects_a_checksum_mismatch_and_cleans_up
---

# Signature

`async fn mock_http_server(body: Vec<u8>) -> String`

# Called by

- [download_model_writes_the_file_and_reports_progress](../../../../../functions/src/llm/provider/llama_cpp_models/download_model_writes_the_file_and_reports_progress.md)
- [download_model_rejects_a_checksum_mismatch_and_cleans_up](../../../../../functions/src/llm/provider/llama_cpp_models/download_model_rejects_a_checksum_mismatch_and_cleans_up.md)