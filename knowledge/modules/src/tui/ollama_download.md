---
type: Rust Module
title: ollama_download
resource: src/tui/ollama_download.rs#L1-L304
generated:
  by: okf-rs/0.2.0
relationships:
  imports:
  - external/super-events-tuievent
  - external/tokio-sync-mpsc-unboundedsender
  - external/tokio-task-joinhandle
  - external/crate-llm-provider-ollama-models
  - external/super
  member_of:
  - packages/crustly
---

# Contains

- [ModelPullProgress](../../../classes/src/tui/ollama_download/ModelPullProgress.md)
- [fraction](../../../functions/src/tui/ollama_download/ModelPullProgress/fraction.md)
- [filter_suggestions](../../../functions/src/tui/ollama_download/filter_suggestions.md)
- [fetch_installed_models](../../../functions/src/tui/ollama_download/fetch_installed_models.md)
- [fetch_installed_models](../../../functions/src/tui/ollama_download/fetch_installed_models-2.md)
- [build_ollama_provider](../../../functions/src/tui/ollama_download/build_ollama_provider.md)
- [build_ollama_provider](../../../functions/src/tui/ollama_download/build_ollama_provider-2.md)
- [spawn_pull](../../../functions/src/tui/ollama_download/spawn_pull.md)
- [spawn_pull](../../../functions/src/tui/ollama_download/spawn_pull-2.md)
- [spawn_delete](../../../functions/src/tui/ollama_download/spawn_delete.md)
- [spawn_delete](../../../functions/src/tui/ollama_download/spawn_delete-2.md)
- [switch_built_provider_applies_per_model_num_ctx_from_config](../../../functions/src/tui/ollama_download/switch_built_provider_applies_per_model_num_ctx_from_config.md)
- [filter_suggestions_empty_query_returns_all_deduped](../../../functions/src/tui/ollama_download/filter_suggestions_empty_query_returns_all_deduped.md)
- [filter_suggestions_matches_substring_case_insensitive](../../../functions/src/tui/ollama_download/filter_suggestions_matches_substring_case_insensitive.md)
- [filter_suggestions_includes_ornith](../../../functions/src/tui/ollama_download/filter_suggestions_includes_ornith.md)
- [pull_progress_fraction](../../../functions/src/tui/ollama_download/pull_progress_fraction.md)

# Imports

- `super::events::TuiEvent`
- `tokio::sync::mpsc::UnboundedSender`
- `tokio::task::JoinHandle`
- `crate::llm::provider::ollama_models`
- `super::*`

# Member of

- [crustly](../../../packages/crustly.md)