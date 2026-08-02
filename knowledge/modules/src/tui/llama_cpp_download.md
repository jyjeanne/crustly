---
type: Rust Module
title: llama_cpp_download
resource: src/tui/llama_cpp_download.rs#L1-L357
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/super-events-tuievent
  - external/std-path-pathbuf
  - external/std-sync-arc-mutex
  - external/tokio-sync-mpsc-unboundedsender
  - external/tokio-task-joinhandle
  - external/crate-llm-provider-llama-cpp-models
  - external/super
  member_of:
  - packages/crustly
---

# Contains

- [LlamaCppModelSummary](../../../classes/src/tui/llama_cpp_download/LlamaCppModelSummary.md)
- [LlamaCppDownloadProgress](../../../classes/src/tui/llama_cpp_download/LlamaCppDownloadProgress.md)
- [fraction](../../../functions/src/tui/llama_cpp_download/LlamaCppDownloadProgress/fraction.md)
- [LlamaCppModelDetails](../../../classes/src/tui/llama_cpp_download/LlamaCppModelDetails.md)
- [list_local](../../../functions/src/tui/llama_cpp_download/list_local.md)
- [list_local](../../../functions/src/tui/llama_cpp_download/list_local-2.md)
- [build_llama_cpp_provider](../../../functions/src/tui/llama_cpp_download/build_llama_cpp_provider.md)
- [build_llama_cpp_provider](../../../functions/src/tui/llama_cpp_download/build_llama_cpp_provider-2.md)
- [spawn_switch](../../../functions/src/tui/llama_cpp_download/spawn_switch.md)
- [spawn_download](../../../functions/src/tui/llama_cpp_download/spawn_download.md)
- [spawn_download](../../../functions/src/tui/llama_cpp_download/spawn_download-2.md)
- [spawn_delete](../../../functions/src/tui/llama_cpp_download/spawn_delete.md)
- [spawn_delete](../../../functions/src/tui/llama_cpp_download/spawn_delete-2.md)
- [filter_local](../../../functions/src/tui/llama_cpp_download/filter_local.md)
- [model](../../../functions/src/tui/llama_cpp_download/model.md)
- [download_progress_fraction](../../../functions/src/tui/llama_cpp_download/download_progress_fraction.md)
- [download_progress_fraction_unknown_total_is_none](../../../functions/src/tui/llama_cpp_download/download_progress_fraction_unknown_total_is_none.md)
- [filter_local_empty_query_returns_all](../../../functions/src/tui/llama_cpp_download/filter_local_empty_query_returns_all.md)
- [filter_local_matches_substring_case_insensitive](../../../functions/src/tui/llama_cpp_download/filter_local_matches_substring_case_insensitive.md)
- [filter_local_no_match_returns_empty](../../../functions/src/tui/llama_cpp_download/filter_local_no_match_returns_empty.md)
- [list_local_without_feature_is_empty](../../../functions/src/tui/llama_cpp_download/list_local_without_feature_is_empty.md)
- [build_provider_without_feature_reports_a_clear_error](../../../functions/src/tui/llama_cpp_download/build_provider_without_feature_reports_a_clear_error.md)

# Imports

- `super::events::TuiEvent`
- `std::path::PathBuf`
- `std::sync::{Arc, Mutex}`
- `tokio::sync::mpsc::UnboundedSender`
- `tokio::task::JoinHandle`
- `crate::llm::provider::llama_cpp_models`
- `super::*`

# Member of

- [crustly](../../../packages/crustly.md)