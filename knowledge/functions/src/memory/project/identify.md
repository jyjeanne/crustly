---
type: Rust Function
title: identify
resource: src/memory/project.rs#L33-L46
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/project/find_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/normalize_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/sha256_hex
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/benches/memory_recall/bench_keyword_recall
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/build_memory_store
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/connect_configured_mcp_servers
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/memory_block
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/run/AgentRun/path_memory_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/run/path_notes_cut_by_the_cap_are_sent_in_a_later_step
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/run/store_with_path_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/save_memory/SaveMemoryTool/save_to_store
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/search_memory/SearchMemoryTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/search_memory/setup
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/search_memory/finds_project_and_global_memories_by_words
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/search_memory/scope_and_category_narrow_the_results
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/a_plain_directory_is_its_own_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/a_subdirectory_resolves_to_its_repository_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/a_linked_worktree_belongs_to_the_main_repository
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/a_submodule_under_a_worktrees_directory_is_its_own_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/the_origin_url_is_read_for_display
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/a_worktree_is_found_through_its_commondir_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/two_clones_of_one_remote_are_two_projects
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/end_session_in_background
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/finish_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/remember_command
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/memory_command
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/reload_memory_rows
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/remember_saves_a_project_memory_and_global_with_the_flag
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/remember_without_text_shows_usage_and_saves_nothing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/forget_asks_first_and_deletes_only_with_yes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/a_save_during_a_response_is_announced_in_the_chat
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/memory_screen_app
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/approve_makes_a_pending_memory_active
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/reject_deletes_a_pending_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/episodes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_acceptance_test/identify_gives_the_same_project_for_the_same_directory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_minor_fixes_test/a_submodule_is_its_own_project_not_its_superprojects
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_transfer_test/from_notes_moves_bullets_keeps_rejected_ones_and_backs_up_first
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_transfer_test/from_notes_without_a_file_does_nothing
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn identify(working_dir: &Path) -> ProjectIdentity`

# Calls

- [find_root](../../../../functions/src/memory/project/find_root.md)
- [normalize_root](../../../../functions/src/memory/project/normalize_root.md)
- [sha256_hex](../../../../functions/src/memory/project/sha256_hex.md)

# Called by

- [bench_keyword_recall](../../../../functions/benches/memory_recall/bench_keyword_recall.md)
- [build_memory_store](../../../../functions/src/cli/build_memory_store.md)
- [cmd_memory](../../../../functions/src/cli/cmd_memory.md)
- [connect_configured_mcp_servers](../../../../functions/src/cli/connect_configured_mcp_servers.md)
- [cmd_run](../../../../functions/src/cli/cmd_run.md)
- [memory_block](../../../../functions/src/llm/agent/service/AgentService/memory_block.md)
- [path_memory_text](../../../../functions/src/llm/agent/service/run/AgentRun/path_memory_text.md)
- [path_notes_cut_by_the_cap_are_sent_in_a_later_step](../../../../functions/src/llm/agent/service/run/path_notes_cut_by_the_cap_are_sent_in_a_later_step.md)
- [store_with_path_memory](../../../../functions/src/llm/agent/service/run/store_with_path_memory.md)
- [save_to_store](../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/save_to_store.md)
- [execute](../../../../functions/src/llm/tools/search_memory/SearchMemoryTool/tool/execute.md)
- [setup](../../../../functions/src/llm/tools/search_memory/setup.md)
- [finds_project_and_global_memories_by_words](../../../../functions/src/llm/tools/search_memory/finds_project_and_global_memories_by_words.md)
- [scope_and_category_narrow_the_results](../../../../functions/src/llm/tools/search_memory/scope_and_category_narrow_the_results.md)
- [a_plain_directory_is_its_own_project](../../../../functions/src/memory/project/a_plain_directory_is_its_own_project.md)
- [a_subdirectory_resolves_to_its_repository_root](../../../../functions/src/memory/project/a_subdirectory_resolves_to_its_repository_root.md)
- [a_linked_worktree_belongs_to_the_main_repository](../../../../functions/src/memory/project/a_linked_worktree_belongs_to_the_main_repository.md)
- [a_submodule_under_a_worktrees_directory_is_its_own_project](../../../../functions/src/memory/project/a_submodule_under_a_worktrees_directory_is_its_own_project.md)
- [the_origin_url_is_read_for_display](../../../../functions/src/memory/project/the_origin_url_is_read_for_display.md)
- [a_worktree_is_found_through_its_commondir_file](../../../../functions/src/memory/project/a_worktree_is_found_through_its_commondir_file.md)
- [two_clones_of_one_remote_are_two_projects](../../../../functions/src/memory/project/two_clones_of_one_remote_are_two_projects.md)
- [end_session_in_background](../../../../functions/src/tui/app/App/end_session_in_background.md)
- [finish_memory](../../../../functions/src/tui/app/App/finish_memory.md)
- [remember_command](../../../../functions/src/tui/app/App/remember_command.md)
- [memory_command](../../../../functions/src/tui/app/App/memory_command.md)
- [reload_memory_rows](../../../../functions/src/tui/app/App/reload_memory_rows.md)
- [remember_saves_a_project_memory_and_global_with_the_flag](../../../../functions/src/tui/app/remember_saves_a_project_memory_and_global_with_the_flag.md)
- [remember_without_text_shows_usage_and_saves_nothing](../../../../functions/src/tui/app/remember_without_text_shows_usage_and_saves_nothing.md)
- [forget_asks_first_and_deletes_only_with_yes](../../../../functions/src/tui/app/forget_asks_first_and_deletes_only_with_yes.md)
- [a_save_during_a_response_is_announced_in_the_chat](../../../../functions/src/tui/app/a_save_during_a_response_is_announced_in_the_chat.md)
- [memory_screen_app](../../../../functions/src/tui/app/memory_screen_app.md)
- [approve_makes_a_pending_memory_active](../../../../functions/src/tui/app/approve_makes_a_pending_memory_active.md)
- [reject_deletes_a_pending_memory](../../../../functions/src/tui/app/reject_deletes_a_pending_memory.md)
- [episodes](../../../../functions/src/tui/app/episodes.md)
- [identify_gives_the_same_project_for_the_same_directory](../../../../functions/tests/memory_acceptance_test/identify_gives_the_same_project_for_the_same_directory.md)
- [a_submodule_is_its_own_project_not_its_superprojects](../../../../functions/tests/memory_minor_fixes_test/a_submodule_is_its_own_project_not_its_superprojects.md)
- [from_notes_moves_bullets_keeps_rejected_ones_and_backs_up_first](../../../../functions/tests/memory_transfer_test/from_notes_moves_bullets_keeps_rejected_ones_and_backs_up_first.md)
- [from_notes_without_a_file_does_nothing](../../../../functions/tests/memory_transfer_test/from_notes_without_a_file_does_nothing.md)