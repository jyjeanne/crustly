---
type: Rust Method
title: default
resource: src/config/mod.rs#L313-L329
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/config/default_episodic_budget
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/default_compaction_threshold
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/default_recall_max_tokens
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/default_pinned_min_importance
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/MemoryEmbeddingsConfig/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/MemoryExtractionConfig/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/MemoryLifecycleConfig/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/MemoryCodeKnowledgeConfig/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/benches/memory_recall/bench_keyword_recall
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/cli/reject_refuses_a_memory_that_is_not_pending
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/Config/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/memory_sanitized_restores_defaults_for_out_of_range_values
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/memory_sanitized_keeps_valid_values
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/memory_test_store
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/run/path_notes_cut_by_the_cap_are_sent_in_a_later_step
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/run/store_with_path_memory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/forget_memory/setup
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/save_memory/store_with_db
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/search_memory/setup
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/memory_test_app
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/memory_screen_app
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_acceptance_test/open
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_anchors_test/setup
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_benchmark_test/run
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_code_knowledge_test/setup
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_code_knowledge_test/without_code_knowledge_nothing_changes
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_consolidate_test/setup
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_consolidate_test/three_episodes_become_one_lesson_with_its_sources
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_consolidate_test/extraction_on
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_consolidate_test/lessons_need_extraction_on
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_extraction_test/enabled_config
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_extraction_test/extraction_is_off_by_default_and_calls_nothing
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_extraction_test/an_episode_in_recent_work_is_not_listed_twice
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_extraction_test/recent_work_from_episodes_keeps_to_its_cap_and_keeps_the_newest
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_minor_fixes_test/store
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_paths_test/store_and_project
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_paths_test/store_and_real_project
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_review_fixes_test/forget_is_refused_in_plan_mode_and_deletes_nothing
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_semantic_test/a_memory_is_recalled_by_meaning_without_a_shared_word
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_semantic_test/an_unrelated_memory_below_the_threshold_is_not_recalled
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_semantic_test/when_the_embedder_is_down_keyword_recall_still_works
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_semantic_test/a_failed_embedding_is_retried_and_then_stored
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_semantic_test/reindex_embeds_memories_saved_before_the_embedder_was_configured
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_semantic_test/two_memories_saying_the_same_thing_are_recalled_once
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_semantic_test/bad_embeddings_settings_fall_back_to_defaults_at_load
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_semantic_test/one_of_two_near_duplicates_is_archived_not_deleted
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_semantic_test/an_edited_memory_is_embedded_again_from_its_new_text
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_semantic_test/an_approved_memory_is_embedded
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_semantic_test/a_clear_semantic_match_beats_a_weak_keyword_match_under_a_tight_budget
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_store_test/store
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_transfer_test/store
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn default() -> Self`

# Calls

- [default_episodic_budget](../../../../../functions/src/config/default_episodic_budget.md)
- [default_compaction_threshold](../../../../../functions/src/config/default_compaction_threshold.md)
- [default_recall_max_tokens](../../../../../functions/src/config/default_recall_max_tokens.md)
- [default_pinned_min_importance](../../../../../functions/src/config/default_pinned_min_importance.md)
- [default](../../../../../functions/src/config/MemoryEmbeddingsConfig/default/default.md)
- [default](../../../../../functions/src/config/MemoryExtractionConfig/default/default.md)
- [default](../../../../../functions/src/config/MemoryLifecycleConfig/default/default.md)
- [default](../../../../../functions/src/config/MemoryCodeKnowledgeConfig/default/default.md)

# Called by

- [bench_keyword_recall](../../../../../functions/benches/memory_recall/bench_keyword_recall.md)
- [reject_refuses_a_memory_that_is_not_pending](../../../../../functions/src/cli/reject_refuses_a_memory_that_is_not_pending.md)
- [default](../../../../../functions/src/config/Config/default/default.md)
- [memory_sanitized_restores_defaults_for_out_of_range_values](../../../../../functions/src/config/memory_sanitized_restores_defaults_for_out_of_range_values.md)
- [memory_sanitized_keeps_valid_values](../../../../../functions/src/config/memory_sanitized_keeps_valid_values.md)
- [memory_test_store](../../../../../functions/src/llm/agent/service/memory_test_store.md)
- [path_notes_cut_by_the_cap_are_sent_in_a_later_step](../../../../../functions/src/llm/agent/service/run/path_notes_cut_by_the_cap_are_sent_in_a_later_step.md)
- [store_with_path_memory](../../../../../functions/src/llm/agent/service/run/store_with_path_memory.md)
- [setup](../../../../../functions/src/llm/tools/forget_memory/setup.md)
- [store_with_db](../../../../../functions/src/llm/tools/save_memory/store_with_db.md)
- [setup](../../../../../functions/src/llm/tools/search_memory/setup.md)
- [memory_test_app](../../../../../functions/src/tui/app/memory_test_app.md)
- [memory_screen_app](../../../../../functions/src/tui/app/memory_screen_app.md)
- [open](../../../../../functions/tests/memory_acceptance_test/open.md)
- [setup](../../../../../functions/tests/memory_anchors_test/setup.md)
- [run](../../../../../functions/tests/memory_benchmark_test/run.md)
- [setup](../../../../../functions/tests/memory_code_knowledge_test/setup.md)
- [without_code_knowledge_nothing_changes](../../../../../functions/tests/memory_code_knowledge_test/without_code_knowledge_nothing_changes.md)
- [setup](../../../../../functions/tests/memory_consolidate_test/setup.md)
- [three_episodes_become_one_lesson_with_its_sources](../../../../../functions/tests/memory_consolidate_test/three_episodes_become_one_lesson_with_its_sources.md)
- [extraction_on](../../../../../functions/tests/memory_consolidate_test/extraction_on.md)
- [lessons_need_extraction_on](../../../../../functions/tests/memory_consolidate_test/lessons_need_extraction_on.md)
- [enabled_config](../../../../../functions/tests/memory_extraction_test/enabled_config.md)
- [extraction_is_off_by_default_and_calls_nothing](../../../../../functions/tests/memory_extraction_test/extraction_is_off_by_default_and_calls_nothing.md)
- [an_episode_in_recent_work_is_not_listed_twice](../../../../../functions/tests/memory_extraction_test/an_episode_in_recent_work_is_not_listed_twice.md)
- [recent_work_from_episodes_keeps_to_its_cap_and_keeps_the_newest](../../../../../functions/tests/memory_extraction_test/recent_work_from_episodes_keeps_to_its_cap_and_keeps_the_newest.md)
- [store](../../../../../functions/tests/memory_minor_fixes_test/store.md)
- [store_and_project](../../../../../functions/tests/memory_paths_test/store_and_project.md)
- [store_and_real_project](../../../../../functions/tests/memory_paths_test/store_and_real_project.md)
- [forget_is_refused_in_plan_mode_and_deletes_nothing](../../../../../functions/tests/memory_review_fixes_test/forget_is_refused_in_plan_mode_and_deletes_nothing.md)
- [a_memory_is_recalled_by_meaning_without_a_shared_word](../../../../../functions/tests/memory_semantic_test/a_memory_is_recalled_by_meaning_without_a_shared_word.md)
- [an_unrelated_memory_below_the_threshold_is_not_recalled](../../../../../functions/tests/memory_semantic_test/an_unrelated_memory_below_the_threshold_is_not_recalled.md)
- [when_the_embedder_is_down_keyword_recall_still_works](../../../../../functions/tests/memory_semantic_test/when_the_embedder_is_down_keyword_recall_still_works.md)
- [a_failed_embedding_is_retried_and_then_stored](../../../../../functions/tests/memory_semantic_test/a_failed_embedding_is_retried_and_then_stored.md)
- [reindex_embeds_memories_saved_before_the_embedder_was_configured](../../../../../functions/tests/memory_semantic_test/reindex_embeds_memories_saved_before_the_embedder_was_configured.md)
- [two_memories_saying_the_same_thing_are_recalled_once](../../../../../functions/tests/memory_semantic_test/two_memories_saying_the_same_thing_are_recalled_once.md)
- [bad_embeddings_settings_fall_back_to_defaults_at_load](../../../../../functions/tests/memory_semantic_test/bad_embeddings_settings_fall_back_to_defaults_at_load.md)
- [one_of_two_near_duplicates_is_archived_not_deleted](../../../../../functions/tests/memory_semantic_test/one_of_two_near_duplicates_is_archived_not_deleted.md)
- [an_edited_memory_is_embedded_again_from_its_new_text](../../../../../functions/tests/memory_semantic_test/an_edited_memory_is_embedded_again_from_its_new_text.md)
- [an_approved_memory_is_embedded](../../../../../functions/tests/memory_semantic_test/an_approved_memory_is_embedded.md)
- [a_clear_semantic_match_beats_a_weak_keyword_match_under_a_tight_budget](../../../../../functions/tests/memory_semantic_test/a_clear_semantic_match_beats_a_weak_keyword_match_under_a_tight_budget.md)
- [store](../../../../../functions/tests/memory_store_test/store.md)
- [store](../../../../../functions/tests/memory_transfer_test/store.md)