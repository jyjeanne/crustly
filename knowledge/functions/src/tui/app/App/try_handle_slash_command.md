---
type: Rust Method
title: try_handle_slash_command
resource: src/tui/app.rs#L2089-L2159
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/events/EventHandler/next
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/open_skills
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/remember_command
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/forget_command
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/open_memory_view
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/memory_command
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/open_mcp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/switch_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/resolve_skill_content
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/send_message
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/tui/app/App/handle_chat_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/unrecognized_slash_word_falls_through_instead_of_being_swallowed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/slash_review_resolves_to_the_builtin_skill
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/slash_review_with_arguments_appends_them_to_the_skill_prompt
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/slash_spec_resolves_to_the_builtin_skill_with_arguments
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/non_slash_message_is_never_treated_as_a_command
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/forget_asks_first_and_deletes_only_with_yes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/memory_lists_the_project_and_global_memories
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/memory_commands_say_so_when_memory_is_off
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/a_save_during_a_response_is_announced_in_the_chat
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/memories_opens_the_screen_with_pending_memories_first
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/scope_and_category_filters_narrow_the_list
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/search_filters_by_words_in_the_content
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/approve_makes_a_pending_memory_active
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/reject_deletes_a_pending_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/forget_asks_first_and_deletes_only_on_y
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/enter_shows_details_with_the_history
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/slash_skills_command_opens_skills_view
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/slash_mcp_command_opens_mcp_view
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/slash_help_command_opens_help_view
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/remember_saves_a_project_memory_and_global_with_the_flag
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/remember_without_text_shows_usage_and_saves_nothing
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn try_handle_slash_command(&mut self, content: &str) -> Result<bool>`

# Calls

- [next](../../../../../functions/src/tui/events/EventHandler/next.md)
- [open_skills](../../../../../functions/src/tui/app/App/open_skills.md)
- [remember_command](../../../../../functions/src/tui/app/App/remember_command.md)
- [forget_command](../../../../../functions/src/tui/app/App/forget_command.md)
- [open_memory_view](../../../../../functions/src/tui/app/App/open_memory_view.md)
- [memory_command](../../../../../functions/src/tui/app/App/memory_command.md)
- [open_mcp](../../../../../functions/src/tui/app/App/open_mcp.md)
- [switch_mode](../../../../../functions/src/tui/app/App/switch_mode.md)
- [resolve_skill_content](../../../../../functions/src/llm/tools/skill/resolve_skill_content.md)
- [send_message](../../../../../functions/src/tui/app/App/send_message.md)

# Called by

- [handle_chat_key](../../../../../functions/src/tui/app/App/handle_chat_key.md)
- [unrecognized_slash_word_falls_through_instead_of_being_swallowed](../../../../../functions/src/tui/app/unrecognized_slash_word_falls_through_instead_of_being_swallowed.md)
- [slash_review_resolves_to_the_builtin_skill](../../../../../functions/src/tui/app/slash_review_resolves_to_the_builtin_skill.md)
- [slash_review_with_arguments_appends_them_to_the_skill_prompt](../../../../../functions/src/tui/app/slash_review_with_arguments_appends_them_to_the_skill_prompt.md)
- [slash_spec_resolves_to_the_builtin_skill_with_arguments](../../../../../functions/src/tui/app/slash_spec_resolves_to_the_builtin_skill_with_arguments.md)
- [non_slash_message_is_never_treated_as_a_command](../../../../../functions/src/tui/app/non_slash_message_is_never_treated_as_a_command.md)
- [forget_asks_first_and_deletes_only_with_yes](../../../../../functions/src/tui/app/forget_asks_first_and_deletes_only_with_yes.md)
- [memory_lists_the_project_and_global_memories](../../../../../functions/src/tui/app/memory_lists_the_project_and_global_memories.md)
- [memory_commands_say_so_when_memory_is_off](../../../../../functions/src/tui/app/memory_commands_say_so_when_memory_is_off.md)
- [a_save_during_a_response_is_announced_in_the_chat](../../../../../functions/src/tui/app/a_save_during_a_response_is_announced_in_the_chat.md)
- [memories_opens_the_screen_with_pending_memories_first](../../../../../functions/src/tui/app/memories_opens_the_screen_with_pending_memories_first.md)
- [scope_and_category_filters_narrow_the_list](../../../../../functions/src/tui/app/scope_and_category_filters_narrow_the_list.md)
- [search_filters_by_words_in_the_content](../../../../../functions/src/tui/app/search_filters_by_words_in_the_content.md)
- [approve_makes_a_pending_memory_active](../../../../../functions/src/tui/app/approve_makes_a_pending_memory_active.md)
- [reject_deletes_a_pending_memory](../../../../../functions/src/tui/app/reject_deletes_a_pending_memory.md)
- [forget_asks_first_and_deletes_only_on_y](../../../../../functions/src/tui/app/forget_asks_first_and_deletes_only_on_y.md)
- [enter_shows_details_with_the_history](../../../../../functions/src/tui/app/enter_shows_details_with_the_history.md)
- [slash_skills_command_opens_skills_view](../../../../../functions/src/tui/app/slash_skills_command_opens_skills_view.md)
- [slash_mcp_command_opens_mcp_view](../../../../../functions/src/tui/app/slash_mcp_command_opens_mcp_view.md)
- [slash_help_command_opens_help_view](../../../../../functions/src/tui/app/slash_help_command_opens_help_view.md)
- [remember_saves_a_project_memory_and_global_with_the_flag](../../../../../functions/src/tui/app/remember_saves_a_project_memory_and_global_with_the_flag.md)
- [remember_without_text_shows_usage_and_saves_nothing](../../../../../functions/src/tui/app/remember_without_text_shows_usage_and_saves_nothing.md)