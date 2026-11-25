---
type: Rust Function
title: mock_http_server
resource: src/llm/provider/llama_cpp_models.rs#L1390-L1427
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/llama_cpp_models/download_model_writes_the_file_and_reports_progress
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/download_model_rejects_a_checksum_mismatch_and_cleans_up
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/download_model_falls_back_to_a_fresh_start_when_the_server_ignores_range
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn mock_http_server(body: Vec<u8>) -> String`

# Called by

- [download_model_writes_the_file_and_reports_progress](../../../../../functions/src/llm/provider/llama_cpp_models/download_model_writes_the_file_and_reports_progress.md)
- [download_model_rejects_a_checksum_mismatch_and_cleans_up](../../../../../functions/src/llm/provider/llama_cpp_models/download_model_rejects_a_checksum_mismatch_and_cleans_up.md)
- [download_model_falls_back_to_a_fresh_start_when_the_server_ignores_range](../../../../../functions/src/llm/provider/llama_cpp_models/download_model_falls_back_to_a_fresh_start_when_the_server_ignores_range.md)