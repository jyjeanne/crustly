---
type: Rust Function
title: to_hex
resource: src/llm/provider/llama_cpp_models.rs#L1147-L1149
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/llama_cpp_models/download_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/download_model_verifies_checksum_across_a_resumed_download
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn to_hex(bytes: &[u8]) -> String`

# Called by

- [download_model](../../../../../functions/src/llm/provider/llama_cpp_models/download_model.md)
- [download_model_verifies_checksum_across_a_resumed_download](../../../../../functions/src/llm/provider/llama_cpp_models/download_model_verifies_checksum_across_a_resumed_download.md)