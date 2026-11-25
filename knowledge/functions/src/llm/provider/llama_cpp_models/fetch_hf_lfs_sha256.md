---
type: Rust Function
title: fetch_hf_lfs_sha256
resource: src/llm/provider/llama_cpp_models.rs#L905-L925
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/llama_cpp_models/resolve_download_source
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn fetch_hf_lfs_sha256( org: &str, repo: &str, file: &str, revision: Option<&str>, ) -> Option<String>`

# Calls

- [find](../../../../../functions/src/memory/MemoryStore/find.md)

# Called by

- [resolve_download_source](../../../../../functions/src/llm/provider/llama_cpp_models/resolve_download_source.md)