---
type: Rust Module
title: memory_extraction_test
resource: tests/memory_extraction_test.rs#L1-L755
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/async-trait-async-trait
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-config-memoryconfig-memoryextractionconfig
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-db-database
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-llm-provider-contentblock-llmrequest-llmresponse-provider-providerstream-result-as-providerresult-stopreason-tokenusage
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-memory-model-memorycategory-memoryscope-memorysource-memorystatus-newmemory-saveoutcome
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-memory-project-id-for-root-projectidentity
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crustly-memory-memorystore-recallrequest
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-collections-vecdeque
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-arc-mutex
    resolved_by: tree-sitter
    confidence: exact
  - target: external/uuid-uuid
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [Scripted](../../classes/tests/memory_extraction_test/Scripted.md)
- [new](../../functions/tests/memory_extraction_test/Scripted/new.md)
- [calls](../../functions/tests/memory_extraction_test/Scripted/calls.md)
- [last_prompt](../../functions/tests/memory_extraction_test/Scripted/last_prompt.md)
- [complete](../../functions/tests/memory_extraction_test/Scripted/provider/complete.md)
- [stream](../../functions/tests/memory_extraction_test/Scripted/provider/stream.md)
- [name](../../functions/tests/memory_extraction_test/Scripted/provider/name.md)
- [default_model](../../functions/tests/memory_extraction_test/Scripted/provider/default_model.md)
- [supported_models](../../functions/tests/memory_extraction_test/Scripted/provider/supported_models.md)
- [context_window](../../functions/tests/memory_extraction_test/Scripted/provider/context_window.md)
- [calculate_cost](../../functions/tests/memory_extraction_test/Scripted/provider/calculate_cost.md)
- [enabled_config](../../functions/tests/memory_extraction_test/enabled_config.md)
- [database](../../functions/tests/memory_extraction_test/database.md)
- [project](../../functions/tests/memory_extraction_test/project.md)
- [session_with](../../functions/tests/memory_extraction_test/session_with.md)
- [chat](../../functions/tests/memory_extraction_test/chat.md)
- [recall](../../functions/tests/memory_extraction_test/recall.md)
- [extraction_is_off_by_default_and_calls_nothing](../../functions/tests/memory_extraction_test/extraction_is_off_by_default_and_calls_nothing.md)
- [a_run_saves_what_the_model_proposes_and_moves_the_watermark](../../functions/tests/memory_extraction_test/a_run_saves_what_the_model_proposes_and_moves_the_watermark.md)
- [below_the_every_n_count_nothing_runs_unless_forced](../../functions/tests/memory_extraction_test/below_the_every_n_count_nothing_runs_unless_forced.md)
- [the_model_is_never_shown_tool_output_or_secrets](../../functions/tests/memory_extraction_test/the_model_is_never_shown_tool_output_or_secrets.md)
- [an_unusable_answer_is_retried_then_skipped](../../functions/tests/memory_extraction_test/an_unusable_answer_is_retried_then_skipped.md)
- [review_holds_memories_until_approved_and_rejection_deletes_them](../../functions/tests/memory_extraction_test/review_holds_memories_until_approved_and_rejection_deletes_them.md)
- [a_session_end_run_writes_an_episode_that_starts_the_next_session](../../functions/tests/memory_extraction_test/a_session_end_run_writes_an_episode_that_starts_the_next_session.md)
- [an_extracted_memory_cannot_replace_a_user_memory_without_user_evidence](../../functions/tests/memory_extraction_test/an_extracted_memory_cannot_replace_a_user_memory_without_user_evidence.md)
- [a_candidate_with_an_unknown_scope_is_dropped](../../functions/tests/memory_extraction_test/a_candidate_with_an_unknown_scope_is_dropped.md)
- [the_background_trigger_runs_once_per_session_at_a_time](../../functions/tests/memory_extraction_test/the_background_trigger_runs_once_per_session_at_a_time.md)
- [an_approved_correction_supersedes_the_memory_it_corrects](../../functions/tests/memory_extraction_test/an_approved_correction_supersedes_the_memory_it_corrects.md)
- [the_user_saying_a_pending_memory_approves_it](../../functions/tests/memory_extraction_test/the_user_saying_a_pending_memory_approves_it.md)
- [a_range_the_provider_always_fails_on_is_skipped_after_three_tries](../../functions/tests/memory_extraction_test/a_range_the_provider_always_fails_on_is_skipped_after_three_tries.md)
- [a_long_backlog_is_read_over_several_runs](../../functions/tests/memory_extraction_test/a_long_backlog_is_read_over_several_runs.md)
- [a_session_end_run_over_a_long_backlog_reads_the_newest_turns](../../functions/tests/memory_extraction_test/a_session_end_run_over_a_long_backlog_reads_the_newest_turns.md)
- [save_episode](../../functions/tests/memory_extraction_test/save_episode.md)
- [first_request_block](../../functions/tests/memory_extraction_test/first_request_block.md)
- [an_episode_in_recent_work_is_not_listed_twice](../../functions/tests/memory_extraction_test/an_episode_in_recent_work_is_not_listed_twice.md)
- [recent_work_from_episodes_keeps_to_its_cap_and_keeps_the_newest](../../functions/tests/memory_extraction_test/recent_work_from_episodes_keeps_to_its_cap_and_keeps_the_newest.md)

# Imports

- `async_trait::async_trait`
- `crustly::config::{MemoryConfig, MemoryExtractionConfig}`
- `crustly::db::Database`
- `crustly::llm::provider::{
    ContentBlock, LLMRequest, LLMResponse, Provider, ProviderStream, Result as ProviderResult,
    StopReason, TokenUsage,
}`
- `crustly::memory::model::{
    MemoryCategory, MemoryScope, MemorySource, MemoryStatus, NewMemory, SaveOutcome,
}`
- `crustly::memory::project::{id_for_root, ProjectIdentity}`
- `crustly::memory::{MemoryStore, RecallRequest}`
- `std::collections::VecDeque`
- `std::path::{Path, PathBuf}`
- `std::sync::{Arc, Mutex}`
- `uuid::Uuid`

# Member of

- [crustly](../../packages/crustly.md)