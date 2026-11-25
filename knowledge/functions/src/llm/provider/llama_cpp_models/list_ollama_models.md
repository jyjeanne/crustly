---
type: Rust Function
title: list_ollama_models
resource: src/llm/provider/llama_cpp_models.rs#L370-L380
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/llama_cpp_models/walk_ollama_manifest_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/parse_ollama_manifest
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/llama_cpp_models/list_all_local_models
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_ollama_models_resolves_library_namespace_display_name
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_ollama_models_keeps_non_library_namespace_in_display_name
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_ollama_models_on_missing_manifests_dir_returns_empty_not_error
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_ollama_models_skips_a_manifest_whose_blob_is_missing
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn list_ollama_models(ollama_models_dir: &Path) -> Vec<LocalGgufModel>`

# Calls

- [walk_ollama_manifest_files](../../../../../functions/src/llm/provider/llama_cpp_models/walk_ollama_manifest_files.md)
- [parse_ollama_manifest](../../../../../functions/src/llm/provider/llama_cpp_models/parse_ollama_manifest.md)

# Called by

- [list_all_local_models](../../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models.md)
- [list_ollama_models_resolves_library_namespace_display_name](../../../../../functions/src/llm/provider/llama_cpp_models/list_ollama_models_resolves_library_namespace_display_name.md)
- [list_ollama_models_keeps_non_library_namespace_in_display_name](../../../../../functions/src/llm/provider/llama_cpp_models/list_ollama_models_keeps_non_library_namespace_in_display_name.md)
- [list_ollama_models_on_missing_manifests_dir_returns_empty_not_error](../../../../../functions/src/llm/provider/llama_cpp_models/list_ollama_models_on_missing_manifests_dir_returns_empty_not_error.md)
- [list_ollama_models_skips_a_manifest_whose_blob_is_missing](../../../../../functions/src/llm/provider/llama_cpp_models/list_ollama_models_skips_a_manifest_whose_blob_is_missing.md)