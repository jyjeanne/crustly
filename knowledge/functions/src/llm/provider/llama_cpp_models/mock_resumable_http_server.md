---
type: Rust Function
title: mock_resumable_http_server
resource: src/llm/provider/llama_cpp_models.rs#L1433-L1492
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/models/PlanTaskStatus/parse
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/llama_cpp_models/download_model_resumes_from_an_existing_partial_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/download_model_verifies_checksum_across_a_resumed_download
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn mock_resumable_http_server(body: Vec<u8>) -> String`

# Calls

- [find](../../../../../functions/src/memory/MemoryStore/find.md)
- [parse](../../../../../functions/src/db/models/PlanTaskStatus/parse.md)

# Called by

- [download_model_resumes_from_an_existing_partial_file](../../../../../functions/src/llm/provider/llama_cpp_models/download_model_resumes_from_an_existing_partial_file.md)
- [download_model_verifies_checksum_across_a_resumed_download](../../../../../functions/src/llm/provider/llama_cpp_models/download_model_verifies_checksum_across_a_resumed_download.md)