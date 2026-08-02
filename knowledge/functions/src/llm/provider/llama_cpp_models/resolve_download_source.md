---
type: Rust Function
title: resolve_download_source
resource: src/llm/provider/llama_cpp_models.rs#L142-L150
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/llama_cpp_models/parse_hf_shorthand
  - functions/src/llm/provider/llama_cpp_models/fetch_hf_lfs_sha256
  called_by:
  - functions/src/cli/cmd_llama_cpp
  - functions/src/llm/provider/llama_cpp_models/resolve_download_source_passes_through_a_direct_url_unchanged
  - functions/src/tui/llama_cpp_download/spawn_download
---

# Signature

`pub async fn resolve_download_source(source: &str) -> Result<(String, Option<String>)>`

# Calls

- [parse_hf_shorthand](../../../../../functions/src/llm/provider/llama_cpp_models/parse_hf_shorthand.md)
- [fetch_hf_lfs_sha256](../../../../../functions/src/llm/provider/llama_cpp_models/fetch_hf_lfs_sha256.md)

# Called by

- [cmd_llama_cpp](../../../../../functions/src/cli/cmd_llama_cpp.md)
- [resolve_download_source_passes_through_a_direct_url_unchanged](../../../../../functions/src/llm/provider/llama_cpp_models/resolve_download_source_passes_through_a_direct_url_unchanged.md)
- [spawn_download](../../../../../functions/src/tui/llama_cpp_download/spawn_download.md)