---
type: Rust Method
title: all
resource: src/memory/mod.rs#L656-L658
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/MemoryRepository/list_all
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/compaction/elide_tool_results_keeps_recent_output_and_frees_tokens
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/compaction/compaction_integration_preserves_last_10_turns
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/middleware/LoopGuard/roundmiddleware/on_tool_calls
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/recall_block_reaches_the_model_but_not_the_database
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/parse_ollama_manifest
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/parse_split_gguf_filename
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_all_local_models_does_not_pair_when_two_candidates_are_ambiguous
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_all_local_models_does_not_pair_across_directories
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/apply_patch/find_subsequence
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/client/namespaced_tool_name_matches_provider_function_name_pattern
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/paths/mentioned_paths
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/recall/unmatched_memories_never_take_budget_when_others_compete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/transfer/export
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanDocument/is_complete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanDocument/next_executable_task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanDocument/next_executable_task_mut
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanDocument/dependencies_satisfied
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/input_is_blank
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/model_download_typing_filters_suggestions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/scope_and_category_filters_narrow_the_list
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/markdown/test_empty_markdown
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_transfer_test/export_then_import_recreates_the_memories_in_a_new_database
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn all(&self) -> Result<Vec<Memory>>`

# Calls

- [list_all](../../../../functions/src/db/repository/memory/MemoryRepository/list_all.md)

# Called by

- [cmd_memory](../../../../functions/src/cli/cmd_memory.md)
- [elide_tool_results_keeps_recent_output_and_frees_tokens](../../../../functions/src/llm/agent/compaction/elide_tool_results_keeps_recent_output_and_frees_tokens.md)
- [compaction_integration_preserves_last_10_turns](../../../../functions/src/llm/agent/compaction/compaction_integration_preserves_last_10_turns.md)
- [on_tool_calls](../../../../functions/src/llm/agent/middleware/LoopGuard/roundmiddleware/on_tool_calls.md)
- [recall_block_reaches_the_model_but_not_the_database](../../../../functions/src/llm/agent/service/recall_block_reaches_the_model_but_not_the_database.md)
- [parse_ollama_manifest](../../../../functions/src/llm/provider/llama_cpp_models/parse_ollama_manifest.md)
- [parse_split_gguf_filename](../../../../functions/src/llm/provider/llama_cpp_models/parse_split_gguf_filename.md)
- [list_all_local_models_does_not_pair_when_two_candidates_are_ambiguous](../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models_does_not_pair_when_two_candidates_are_ambiguous.md)
- [list_all_local_models_does_not_pair_across_directories](../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models_does_not_pair_across_directories.md)
- [find_subsequence](../../../../functions/src/llm/tools/apply_patch/find_subsequence.md)
- [namespaced_tool_name_matches_provider_function_name_pattern](../../../../functions/src/mcp/client/namespaced_tool_name_matches_provider_function_name_pattern.md)
- [mentioned_paths](../../../../functions/src/memory/paths/mentioned_paths.md)
- [unmatched_memories_never_take_budget_when_others_compete](../../../../functions/src/memory/recall/unmatched_memories_never_take_budget_when_others_compete.md)
- [export](../../../../functions/src/memory/transfer/export.md)
- [is_complete](../../../../functions/src/plan/PlanDocument/is_complete.md)
- [next_executable_task](../../../../functions/src/plan/PlanDocument/next_executable_task.md)
- [next_executable_task_mut](../../../../functions/src/plan/PlanDocument/next_executable_task_mut.md)
- [dependencies_satisfied](../../../../functions/src/plan/PlanDocument/dependencies_satisfied.md)
- [input_is_blank](../../../../functions/src/tui/app/App/input_is_blank.md)
- [model_download_typing_filters_suggestions](../../../../functions/src/tui/app/model_download_typing_filters_suggestions.md)
- [scope_and_category_filters_narrow_the_list](../../../../functions/src/tui/app/scope_and_category_filters_narrow_the_list.md)
- [test_empty_markdown](../../../../functions/src/tui/markdown/test_empty_markdown.md)
- [export_then_import_recreates_the_memories_in_a_new_database](../../../../functions/tests/memory_transfer_test/export_then_import_recreates_the_memories_in_a_new_database.md)