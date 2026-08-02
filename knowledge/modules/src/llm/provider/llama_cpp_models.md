---
type: Rust Module
title: llama_cpp_models
resource: src/llm/provider/llama_cpp_models.rs#L1-L447
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/anyhow-context-result
  - external/futures-streamext-as
  - external/sha2-digest-sha256
  - external/std-path-path-pathbuf
  - external/tokio-io-asyncwriteext-as
  - external/tokio-sync-mpsc-unboundedsender
  - external/super
  - external/tokio-io-asyncreadext-asyncwriteext
  member_of:
  - packages/crustly
---

# Contains

- [LocalGgufModel](../../../../classes/src/llm/provider/llama_cpp_models/LocalGgufModel.md)
- [DownloadProgress](../../../../classes/src/llm/provider/llama_cpp_models/DownloadProgress.md)
- [fraction](../../../../functions/src/llm/provider/llama_cpp_models/DownloadProgress/fraction.md)
- [quantization_hint_from_filename](../../../../functions/src/llm/provider/llama_cpp_models/quantization_hint_from_filename.md)
- [list_local_models](../../../../functions/src/llm/provider/llama_cpp_models/list_local_models.md)
- [parse_hf_shorthand](../../../../functions/src/llm/provider/llama_cpp_models/parse_hf_shorthand.md)
- [resolve_download_source](../../../../functions/src/llm/provider/llama_cpp_models/resolve_download_source.md)
- [fetch_hf_lfs_sha256](../../../../functions/src/llm/provider/llama_cpp_models/fetch_hf_lfs_sha256.md)
- [download_model](../../../../functions/src/llm/provider/llama_cpp_models/download_model.md)
- [to_hex](../../../../functions/src/llm/provider/llama_cpp_models/to_hex.md)
- [delete_model](../../../../functions/src/llm/provider/llama_cpp_models/delete_model.md)
- [quantization_hint_recognizes_common_tags](../../../../functions/src/llm/provider/llama_cpp_models/quantization_hint_recognizes_common_tags.md)
- [quantization_hint_none_for_unrecognized_filename](../../../../functions/src/llm/provider/llama_cpp_models/quantization_hint_none_for_unrecognized_filename.md)
- [quantization_hint_is_case_insensitive](../../../../functions/src/llm/provider/llama_cpp_models/quantization_hint_is_case_insensitive.md)
- [parse_hf_shorthand_extracts_org_repo_file](../../../../functions/src/llm/provider/llama_cpp_models/parse_hf_shorthand_extracts_org_repo_file.md)
- [parse_hf_shorthand_none_for_direct_url](../../../../functions/src/llm/provider/llama_cpp_models/parse_hf_shorthand_none_for_direct_url.md)
- [parse_hf_shorthand_none_for_malformed_shorthand](../../../../functions/src/llm/provider/llama_cpp_models/parse_hf_shorthand_none_for_malformed_shorthand.md)
- [list_local_models_on_nonexistent_dir_returns_empty_not_error](../../../../functions/src/llm/provider/llama_cpp_models/list_local_models_on_nonexistent_dir_returns_empty_not_error.md)
- [list_local_models_only_lists_gguf_files](../../../../functions/src/llm/provider/llama_cpp_models/list_local_models_only_lists_gguf_files.md)
- [resolve_download_source_passes_through_a_direct_url_unchanged](../../../../functions/src/llm/provider/llama_cpp_models/resolve_download_source_passes_through_a_direct_url_unchanged.md)
- [download_model_writes_the_file_and_reports_progress](../../../../functions/src/llm/provider/llama_cpp_models/download_model_writes_the_file_and_reports_progress.md)
- [download_model_rejects_a_checksum_mismatch_and_cleans_up](../../../../functions/src/llm/provider/llama_cpp_models/download_model_rejects_a_checksum_mismatch_and_cleans_up.md)
- [mock_http_server](../../../../functions/src/llm/provider/llama_cpp_models/mock_http_server.md)

# Imports

- `anyhow::{Context, Result}`
- `futures::StreamExt as _`
- `sha2::{Digest, Sha256}`
- `std::path::{Path, PathBuf}`
- `tokio::io::AsyncWriteExt as _`
- `tokio::sync::mpsc::UnboundedSender`
- `super::*`
- `tokio::io::{AsyncReadExt, AsyncWriteExt}`

# Member of

- [crustly](../../../../packages/crustly.md)