---
type: Rust Function
title: list_all_local_models
resource: src/llm/provider/llama_cpp_models.rs#L277-L331
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/llama_cpp_models/list_local_models
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_ollama_models
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/deduplicate_and_merge
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/pair_mmproj_files
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_llama_cpp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/llama_cpp_doctor_findings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_all_local_models_merges_models_dir_and_extra_paths
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_all_local_models_with_no_ollama_dir_does_not_scan_ollama
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_all_local_models_merges_two_manifest_tags_sharing_one_blob
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_all_local_models_collapses_a_symlink_to_an_already_found_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_all_local_models_collapses_a_split_gguf_group
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_all_local_models_handles_a_partial_split_group_honestly
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_all_local_models_pairs_a_base_model_with_its_mmproj_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_all_local_models_leaves_an_unmatched_mmproj_file_standing_alone
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_all_local_models_does_not_pair_when_two_candidates_are_ambiguous
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_all_local_models_does_not_pair_across_directories
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_all_local_models_does_not_group_unrelated_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/llama_cpp_download/list_local
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn list_all_local_models( models_dir: &Path, extra_model_paths: &[PathBuf], ollama_models_dir: Option<&Path>, ) -> Result<Vec<LocalGgufModel>>`

# Calls

- [list_local_models](../../../../../functions/src/llm/provider/llama_cpp_models/list_local_models.md)
- [list_ollama_models](../../../../../functions/src/llm/provider/llama_cpp_models/list_ollama_models.md)
- [deduplicate_and_merge](../../../../../functions/src/llm/provider/llama_cpp_models/deduplicate_and_merge.md)
- [pair_mmproj_files](../../../../../functions/src/llm/provider/llama_cpp_models/pair_mmproj_files.md)

# Called by

- [cmd_llama_cpp](../../../../../functions/src/cli/cmd_llama_cpp.md)
- [llama_cpp_doctor_findings](../../../../../functions/src/cli/llama_cpp_doctor_findings.md)
- [list_all_local_models_merges_models_dir_and_extra_paths](../../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models_merges_models_dir_and_extra_paths.md)
- [list_all_local_models_with_no_ollama_dir_does_not_scan_ollama](../../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models_with_no_ollama_dir_does_not_scan_ollama.md)
- [list_all_local_models_merges_two_manifest_tags_sharing_one_blob](../../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models_merges_two_manifest_tags_sharing_one_blob.md)
- [list_all_local_models_collapses_a_symlink_to_an_already_found_file](../../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models_collapses_a_symlink_to_an_already_found_file.md)
- [list_all_local_models_collapses_a_split_gguf_group](../../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models_collapses_a_split_gguf_group.md)
- [list_all_local_models_handles_a_partial_split_group_honestly](../../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models_handles_a_partial_split_group_honestly.md)
- [list_all_local_models_pairs_a_base_model_with_its_mmproj_file](../../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models_pairs_a_base_model_with_its_mmproj_file.md)
- [list_all_local_models_leaves_an_unmatched_mmproj_file_standing_alone](../../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models_leaves_an_unmatched_mmproj_file_standing_alone.md)
- [list_all_local_models_does_not_pair_when_two_candidates_are_ambiguous](../../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models_does_not_pair_when_two_candidates_are_ambiguous.md)
- [list_all_local_models_does_not_pair_across_directories](../../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models_does_not_pair_across_directories.md)
- [list_all_local_models_does_not_group_unrelated_files](../../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models_does_not_group_unrelated_files.md)
- [list_local](../../../../../functions/src/tui/llama_cpp_download/list_local.md)