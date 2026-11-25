---
type: Rust Function
title: write_fake_ollama_model
resource: src/llm/provider/llama_cpp_models.rs#L1607-L1635
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/project/write
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/llama_cpp_models/list_ollama_models_resolves_library_namespace_display_name
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_ollama_models_keeps_non_library_namespace_in_display_name
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_all_local_models_merges_two_manifest_tags_sharing_one_blob
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn write_fake_ollama_model( root: &Path, host: &str, namespace: &str, model: &str, tag: &str, hex_digest: &str, blob_body: &[u8], )`

# Calls

- [write](../../../../../functions/src/memory/project/write.md)

# Called by

- [list_ollama_models_resolves_library_namespace_display_name](../../../../../functions/src/llm/provider/llama_cpp_models/list_ollama_models_resolves_library_namespace_display_name.md)
- [list_ollama_models_keeps_non_library_namespace_in_display_name](../../../../../functions/src/llm/provider/llama_cpp_models/list_ollama_models_keeps_non_library_namespace_in_display_name.md)
- [list_all_local_models_merges_two_manifest_tags_sharing_one_blob](../../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models_merges_two_manifest_tags_sharing_one_blob.md)