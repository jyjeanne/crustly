---
type: Rust Module
title: llama_cpp_models
resource: src/llm/provider/llama_cpp_models.rs#L1-L2135
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/anyhow-context-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/futures-streamext-as
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-deserialize
    resolved_by: tree-sitter
    confidence: exact
  - target: external/sha2-digest-sha256
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-collections-hashmap
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tokio-io-asyncreadext-as-asyncwriteext-as
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tokio-sync-mpsc-unboundedsender
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tokio-io-asyncreadext-asyncwriteext
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [estimate_default_context_length](../../../../functions/src/llm/provider/llama_cpp_models/estimate_default_context_length.md)
- [LocalGgufModel](../../../../classes/src/llm/provider/llama_cpp_models/LocalGgufModel.md)
- [DownloadProgress](../../../../classes/src/llm/provider/llama_cpp_models/DownloadProgress.md)
- [fraction](../../../../functions/src/llm/provider/llama_cpp_models/DownloadProgress/fraction.md)
- [quantization_hint_from_filename](../../../../functions/src/llm/provider/llama_cpp_models/quantization_hint_from_filename.md)
- [filename_suggests_mmproj](../../../../functions/src/llm/provider/llama_cpp_models/filename_suggests_mmproj.md)
- [list_local_models](../../../../functions/src/llm/provider/llama_cpp_models/list_local_models.md)
- [list_all_local_models](../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models.md)
- [OllamaManifestLayer](../../../../classes/src/llm/provider/llama_cpp_models/OllamaManifestLayer.md)
- [OllamaManifest](../../../../classes/src/llm/provider/llama_cpp_models/OllamaManifest.md)
- [list_ollama_models](../../../../functions/src/llm/provider/llama_cpp_models/list_ollama_models.md)
- [walk_ollama_manifest_files](../../../../functions/src/llm/provider/llama_cpp_models/walk_ollama_manifest_files.md)
- [parse_ollama_manifest](../../../../functions/src/llm/provider/llama_cpp_models/parse_ollama_manifest.md)
- [parse_split_gguf_filename](../../../../functions/src/llm/provider/llama_cpp_models/parse_split_gguf_filename.md)
- [deduplicate_and_merge](../../../../functions/src/llm/provider/llama_cpp_models/deduplicate_and_merge.md)
- [is_richer](../../../../functions/src/llm/provider/llama_cpp_models/is_richer.md)
- [merge_by_canonical_path](../../../../functions/src/llm/provider/llama_cpp_models/merge_by_canonical_path.md)
- [merge_split_gguf_groups](../../../../functions/src/llm/provider/llama_cpp_models/merge_split_gguf_groups.md)
- [mmproj_core_name](../../../../functions/src/llm/provider/llama_cpp_models/mmproj_core_name.md)
- [pair_mmproj_files](../../../../functions/src/llm/provider/llama_cpp_models/pair_mmproj_files.md)
- [HardwareFit](../../../../classes/src/llm/provider/llama_cpp_models/HardwareFit.md)
- [hardware_fit](../../../../functions/src/llm/provider/llama_cpp_models/hardware_fit.md)
- [sort_by_fit](../../../../functions/src/llm/provider/llama_cpp_models/sort_by_fit.md)
- [rank](../../../../functions/src/llm/provider/llama_cpp_models/rank.md)
- [best_fitting_model](../../../../functions/src/llm/provider/llama_cpp_models/best_fitting_model.md)
- [parse_hf_shorthand](../../../../functions/src/llm/provider/llama_cpp_models/parse_hf_shorthand.md)
- [resolve_download_source](../../../../functions/src/llm/provider/llama_cpp_models/resolve_download_source.md)
- [fetch_hf_lfs_sha256](../../../../functions/src/llm/provider/llama_cpp_models/fetch_hf_lfs_sha256.md)
- [available_space_at](../../../../functions/src/llm/provider/llama_cpp_models/available_space_at.md)
- [check_disk_space](../../../../functions/src/llm/provider/llama_cpp_models/check_disk_space.md)
- [download_model](../../../../functions/src/llm/provider/llama_cpp_models/download_model.md)
- [to_hex](../../../../functions/src/llm/provider/llama_cpp_models/to_hex.md)
- [delete_model](../../../../functions/src/llm/provider/llama_cpp_models/delete_model.md)
- [quantization_hint_recognizes_common_tags](../../../../functions/src/llm/provider/llama_cpp_models/quantization_hint_recognizes_common_tags.md)
- [quantization_hint_none_for_unrecognized_filename](../../../../functions/src/llm/provider/llama_cpp_models/quantization_hint_none_for_unrecognized_filename.md)
- [quantization_hint_is_case_insensitive](../../../../functions/src/llm/provider/llama_cpp_models/quantization_hint_is_case_insensitive.md)
- [parse_hf_shorthand_extracts_org_repo_file](../../../../functions/src/llm/provider/llama_cpp_models/parse_hf_shorthand_extracts_org_repo_file.md)
- [parse_hf_shorthand_extracts_a_pinned_revision](../../../../functions/src/llm/provider/llama_cpp_models/parse_hf_shorthand_extracts_a_pinned_revision.md)
- [parse_hf_shorthand_none_for_direct_url](../../../../functions/src/llm/provider/llama_cpp_models/parse_hf_shorthand_none_for_direct_url.md)
- [parse_hf_shorthand_none_for_malformed_shorthand](../../../../functions/src/llm/provider/llama_cpp_models/parse_hf_shorthand_none_for_malformed_shorthand.md)
- [list_local_models_on_nonexistent_dir_returns_empty_not_error](../../../../functions/src/llm/provider/llama_cpp_models/list_local_models_on_nonexistent_dir_returns_empty_not_error.md)
- [list_local_models_only_lists_gguf_files](../../../../functions/src/llm/provider/llama_cpp_models/list_local_models_only_lists_gguf_files.md)
- [resolve_download_source_passes_through_a_direct_url_unchanged](../../../../functions/src/llm/provider/llama_cpp_models/resolve_download_source_passes_through_a_direct_url_unchanged.md)
- [resolve_download_source_uses_the_pinned_revision_in_the_url](../../../../functions/src/llm/provider/llama_cpp_models/resolve_download_source_uses_the_pinned_revision_in_the_url.md)
- [resolve_download_source_defaults_to_main_without_a_revision](../../../../functions/src/llm/provider/llama_cpp_models/resolve_download_source_defaults_to_main_without_a_revision.md)
- [check_disk_space_rejects_when_available_is_below_needed_plus_margin](../../../../functions/src/llm/provider/llama_cpp_models/check_disk_space_rejects_when_available_is_below_needed_plus_margin.md)
- [check_disk_space_allows_when_comfortably_available](../../../../functions/src/llm/provider/llama_cpp_models/check_disk_space_allows_when_comfortably_available.md)
- [check_disk_space_skips_when_available_is_unknown](../../../../functions/src/llm/provider/llama_cpp_models/check_disk_space_skips_when_available_is_unknown.md)
- [available_space_at_a_real_directory_returns_a_plausible_value_or_none](../../../../functions/src/llm/provider/llama_cpp_models/available_space_at_a_real_directory_returns_a_plausible_value_or_none.md)
- [available_space_at_walks_up_to_the_nearest_existing_ancestor](../../../../functions/src/llm/provider/llama_cpp_models/available_space_at_walks_up_to_the_nearest_existing_ancestor.md)
- [download_model_writes_the_file_and_reports_progress](../../../../functions/src/llm/provider/llama_cpp_models/download_model_writes_the_file_and_reports_progress.md)
- [download_model_rejects_a_checksum_mismatch_and_cleans_up](../../../../functions/src/llm/provider/llama_cpp_models/download_model_rejects_a_checksum_mismatch_and_cleans_up.md)
- [mock_http_server](../../../../functions/src/llm/provider/llama_cpp_models/mock_http_server.md)
- [mock_resumable_http_server](../../../../functions/src/llm/provider/llama_cpp_models/mock_resumable_http_server.md)
- [download_model_resumes_from_an_existing_partial_file](../../../../functions/src/llm/provider/llama_cpp_models/download_model_resumes_from_an_existing_partial_file.md)
- [download_model_verifies_checksum_across_a_resumed_download](../../../../functions/src/llm/provider/llama_cpp_models/download_model_verifies_checksum_across_a_resumed_download.md)
- [download_model_falls_back_to_a_fresh_start_when_the_server_ignores_range](../../../../functions/src/llm/provider/llama_cpp_models/download_model_falls_back_to_a_fresh_start_when_the_server_ignores_range.md)
- [list_all_local_models_merges_models_dir_and_extra_paths](../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models_merges_models_dir_and_extra_paths.md)
- [list_all_local_models_with_no_ollama_dir_does_not_scan_ollama](../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models_with_no_ollama_dir_does_not_scan_ollama.md)
- [write_fake_ollama_model](../../../../functions/src/llm/provider/llama_cpp_models/write_fake_ollama_model.md)
- [list_ollama_models_resolves_library_namespace_display_name](../../../../functions/src/llm/provider/llama_cpp_models/list_ollama_models_resolves_library_namespace_display_name.md)
- [list_ollama_models_keeps_non_library_namespace_in_display_name](../../../../functions/src/llm/provider/llama_cpp_models/list_ollama_models_keeps_non_library_namespace_in_display_name.md)
- [list_ollama_models_on_missing_manifests_dir_returns_empty_not_error](../../../../functions/src/llm/provider/llama_cpp_models/list_ollama_models_on_missing_manifests_dir_returns_empty_not_error.md)
- [list_ollama_models_skips_a_manifest_whose_blob_is_missing](../../../../functions/src/llm/provider/llama_cpp_models/list_ollama_models_skips_a_manifest_whose_blob_is_missing.md)
- [list_all_local_models_merges_two_manifest_tags_sharing_one_blob](../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models_merges_two_manifest_tags_sharing_one_blob.md)
- [list_all_local_models_collapses_a_symlink_to_an_already_found_file](../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models_collapses_a_symlink_to_an_already_found_file.md)
- [parse_split_gguf_filename_matches_the_convention](../../../../functions/src/llm/provider/llama_cpp_models/parse_split_gguf_filename_matches_the_convention.md)
- [parse_split_gguf_filename_rejects_mismatched_widths](../../../../functions/src/llm/provider/llama_cpp_models/parse_split_gguf_filename_rejects_mismatched_widths.md)
- [parse_split_gguf_filename_rejects_non_split_names_and_out_of_range_parts](../../../../functions/src/llm/provider/llama_cpp_models/parse_split_gguf_filename_rejects_non_split_names_and_out_of_range_parts.md)
- [list_all_local_models_collapses_a_split_gguf_group](../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models_collapses_a_split_gguf_group.md)
- [merge_split_gguf_groups_prefers_the_first_part_with_a_parsed_header_on_a_tie](../../../../functions/src/llm/provider/llama_cpp_models/merge_split_gguf_groups_prefers_the_first_part_with_a_parsed_header_on_a_tie.md)
- [merge_split_gguf_groups_falls_back_to_part_one_when_no_part_has_a_header](../../../../functions/src/llm/provider/llama_cpp_models/merge_split_gguf_groups_falls_back_to_part_one_when_no_part_has_a_header.md)
- [list_all_local_models_handles_a_partial_split_group_honestly](../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models_handles_a_partial_split_group_honestly.md)
- [mmproj_core_name_reduces_a_base_model_and_its_projector_identically](../../../../functions/src/llm/provider/llama_cpp_models/mmproj_core_name_reduces_a_base_model_and_its_projector_identically.md)
- [list_all_local_models_pairs_a_base_model_with_its_mmproj_file](../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models_pairs_a_base_model_with_its_mmproj_file.md)
- [list_all_local_models_leaves_an_unmatched_mmproj_file_standing_alone](../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models_leaves_an_unmatched_mmproj_file_standing_alone.md)
- [list_all_local_models_does_not_pair_when_two_candidates_are_ambiguous](../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models_does_not_pair_when_two_candidates_are_ambiguous.md)
- [list_all_local_models_does_not_pair_across_directories](../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models_does_not_pair_across_directories.md)
- [list_all_local_models_does_not_group_unrelated_files](../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models_does_not_group_unrelated_files.md)
- [fixture_model](../../../../functions/src/llm/provider/llama_cpp_models/fixture_model.md)
- [hardware_fit_reports_fits_comfortably_under_budget](../../../../functions/src/llm/provider/llama_cpp_models/hardware_fit_reports_fits_comfortably_under_budget.md)
- [hardware_fit_reports_tight_near_the_threshold](../../../../functions/src/llm/provider/llama_cpp_models/hardware_fit_reports_tight_near_the_threshold.md)
- [hardware_fit_reports_wont_fit_over_budget](../../../../functions/src/llm/provider/llama_cpp_models/hardware_fit_reports_wont_fit_over_budget.md)
- [hardware_fit_is_unknown_when_estimate_is_missing](../../../../functions/src/llm/provider/llama_cpp_models/hardware_fit_is_unknown_when_estimate_is_missing.md)
- [hardware_fit_is_unknown_when_budget_is_missing](../../../../functions/src/llm/provider/llama_cpp_models/hardware_fit_is_unknown_when_budget_is_missing.md)
- [sort_by_fit_orders_fits_before_tight_before_wont_fit_before_unknown](../../../../functions/src/llm/provider/llama_cpp_models/sort_by_fit_orders_fits_before_tight_before_wont_fit_before_unknown.md)
- [sort_by_fit_orders_largest_first_within_a_category](../../../../functions/src/llm/provider/llama_cpp_models/sort_by_fit_orders_largest_first_within_a_category.md)
- [best_fitting_model_picks_the_largest_that_fits_or_is_tight](../../../../functions/src/llm/provider/llama_cpp_models/best_fitting_model_picks_the_largest_that_fits_or_is_tight.md)
- [best_fitting_model_is_none_when_nothing_fits](../../../../functions/src/llm/provider/llama_cpp_models/best_fitting_model_is_none_when_nothing_fits.md)
- [best_fitting_model_is_none_for_an_empty_list](../../../../functions/src/llm/provider/llama_cpp_models/best_fitting_model_is_none_for_an_empty_list.md)

# Imports

- `anyhow::{Context, Result}`
- `futures::StreamExt as _`
- `serde::Deserialize`
- `sha2::{Digest, Sha256}`
- `std::collections::HashMap`
- `std::path::{Path, PathBuf}`
- `tokio::io::{AsyncReadExt as _, AsyncWriteExt as _}`
- `tokio::sync::mpsc::UnboundedSender`
- `super::*`
- `tokio::io::{AsyncReadExt, AsyncWriteExt}`

# Member of

- [crustly](../../../../packages/crustly.md)