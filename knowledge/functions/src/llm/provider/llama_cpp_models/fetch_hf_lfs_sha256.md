---
type: Rust Function
title: fetch_hf_lfs_sha256
resource: src/llm/provider/llama_cpp_models.rs#L157-L169
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/provider/llama_cpp_models/resolve_download_source
---

# Signature

`async fn fetch_hf_lfs_sha256(org: &str, repo: &str, file: &str) -> Option<String>`

# Called by

- [resolve_download_source](../../../../../functions/src/llm/provider/llama_cpp_models/resolve_download_source.md)