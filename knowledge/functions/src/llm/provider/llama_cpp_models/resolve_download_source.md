---
type: Rust Function
title: resolve_download_source
resource: src/llm/provider/llama_cpp_models.rs#L877-L886
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/llama_cpp_models/parse_hf_shorthand
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/fetch_hf_lfs_sha256
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_llama_cpp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/resolve_download_source_passes_through_a_direct_url_unchanged
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/resolve_download_source_uses_the_pinned_revision_in_the_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/resolve_download_source_defaults_to_main_without_a_revision
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/llama_cpp_download/spawn_download
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn resolve_download_source(source: &str) -> Result<(String, Option<String>)>`

# Calls

- [parse_hf_shorthand](../../../../../functions/src/llm/provider/llama_cpp_models/parse_hf_shorthand.md)
- [fetch_hf_lfs_sha256](../../../../../functions/src/llm/provider/llama_cpp_models/fetch_hf_lfs_sha256.md)

# Called by

- [cmd_llama_cpp](../../../../../functions/src/cli/cmd_llama_cpp.md)
- [resolve_download_source_passes_through_a_direct_url_unchanged](../../../../../functions/src/llm/provider/llama_cpp_models/resolve_download_source_passes_through_a_direct_url_unchanged.md)
- [resolve_download_source_uses_the_pinned_revision_in_the_url](../../../../../functions/src/llm/provider/llama_cpp_models/resolve_download_source_uses_the_pinned_revision_in_the_url.md)
- [resolve_download_source_defaults_to_main_without_a_revision](../../../../../functions/src/llm/provider/llama_cpp_models/resolve_download_source_defaults_to_main_without_a_revision.md)
- [spawn_download](../../../../../functions/src/tui/llama_cpp_download/spawn_download.md)