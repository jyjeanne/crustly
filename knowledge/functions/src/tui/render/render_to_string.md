---
type: Rust Function
title: render_to_string
resource: src/tui/render.rs#L3183-L3197
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/render/render
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/render/header_shows_ollama_provider_badge_and_tokens_per_second
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/header_omits_tokens_per_second_when_unavailable
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/status_bar_shows_interactive_by_default
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/status_bar_shows_full_auto_when_active
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/skills_view_shows_name_and_description
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/skills_view_shows_empty_state_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/mcp_view_shows_connected_server_with_tool_count
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/mcp_view_shows_connection_error
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/mcp_view_shows_empty_state_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/model_download_dialog_shows_prompt_and_suggestions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/model_download_progress_shows_status_and_bar
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/model_download_confirm_delete_shows_prompt
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/model_download_deleting_shows_status
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/llama_cpp_models_dialog_shows_prompt_and_local_models
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/llama_cpp_models_dialog_shows_hardware_detecting_state
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/llama_cpp_models_dialog_shows_hardware_summary_once_detected
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/llama_cpp_models_dialog_shows_display_name_not_the_raw_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/llama_cpp_models_dialog_shows_loading_state
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/llama_cpp_download_progress_shows_status_and_bar
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/llama_cpp_confirm_delete_shows_prompt
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/llama_cpp_switching_shows_loading_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/chat_input_renders_textarea_contents_and_hint
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/model_info_panel_shows_provider_model_and_context_window
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/model_info_panel_shows_last_response_perf_metrics
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/help_screen_lists_commands_from_every_section
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/help_screen_shows_shift_enter_when_kitty_protocol_active
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/chat_shows_pending_plan_banner_only_while_awaiting_approval
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/chat_message_thinking_block_toggles_between_collapsed_and_expanded
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/chat_message_perf_footer_reports_cold_and_warm_starts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/chat_shows_streaming_response_and_processing_indicator
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/chat_follows_the_end_of_a_reply_whose_paragraphs_wrap
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/scrolling_up_hides_the_end_and_says_how_to_return
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/an_empty_chat_shows_ferris_and_a_welcome
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/ferris_works_while_the_model_thinks
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/a_new_session_costs_zero_not_minus_zero
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/plan_mode_shows_full_document_with_tasks_and_criteria
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/plan_mode_shows_empty_state_without_a_plan
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/approval_dialog_shows_tool_name_capabilities_and_summarized_params
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/approval_dialog_details_view_shows_pretty_printed_json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/the_memories_screen_shows_its_filters_and_the_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/the_memories_screen_scrolls_to_the_selection
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn render_to_string(app: &App, width: u16, height: u16) -> String`

# Calls

- [render](../../../../functions/src/tui/render/render.md)

# Called by

- [header_shows_ollama_provider_badge_and_tokens_per_second](../../../../functions/src/tui/render/header_shows_ollama_provider_badge_and_tokens_per_second.md)
- [header_omits_tokens_per_second_when_unavailable](../../../../functions/src/tui/render/header_omits_tokens_per_second_when_unavailable.md)
- [status_bar_shows_interactive_by_default](../../../../functions/src/tui/render/status_bar_shows_interactive_by_default.md)
- [status_bar_shows_full_auto_when_active](../../../../functions/src/tui/render/status_bar_shows_full_auto_when_active.md)
- [skills_view_shows_name_and_description](../../../../functions/src/tui/render/skills_view_shows_name_and_description.md)
- [skills_view_shows_empty_state_message](../../../../functions/src/tui/render/skills_view_shows_empty_state_message.md)
- [mcp_view_shows_connected_server_with_tool_count](../../../../functions/src/tui/render/mcp_view_shows_connected_server_with_tool_count.md)
- [mcp_view_shows_connection_error](../../../../functions/src/tui/render/mcp_view_shows_connection_error.md)
- [mcp_view_shows_empty_state_message](../../../../functions/src/tui/render/mcp_view_shows_empty_state_message.md)
- [model_download_dialog_shows_prompt_and_suggestions](../../../../functions/src/tui/render/model_download_dialog_shows_prompt_and_suggestions.md)
- [model_download_progress_shows_status_and_bar](../../../../functions/src/tui/render/model_download_progress_shows_status_and_bar.md)
- [model_download_confirm_delete_shows_prompt](../../../../functions/src/tui/render/model_download_confirm_delete_shows_prompt.md)
- [model_download_deleting_shows_status](../../../../functions/src/tui/render/model_download_deleting_shows_status.md)
- [llama_cpp_models_dialog_shows_prompt_and_local_models](../../../../functions/src/tui/render/llama_cpp_models_dialog_shows_prompt_and_local_models.md)
- [llama_cpp_models_dialog_shows_hardware_detecting_state](../../../../functions/src/tui/render/llama_cpp_models_dialog_shows_hardware_detecting_state.md)
- [llama_cpp_models_dialog_shows_hardware_summary_once_detected](../../../../functions/src/tui/render/llama_cpp_models_dialog_shows_hardware_summary_once_detected.md)
- [llama_cpp_models_dialog_shows_display_name_not_the_raw_path](../../../../functions/src/tui/render/llama_cpp_models_dialog_shows_display_name_not_the_raw_path.md)
- [llama_cpp_models_dialog_shows_loading_state](../../../../functions/src/tui/render/llama_cpp_models_dialog_shows_loading_state.md)
- [llama_cpp_download_progress_shows_status_and_bar](../../../../functions/src/tui/render/llama_cpp_download_progress_shows_status_and_bar.md)
- [llama_cpp_confirm_delete_shows_prompt](../../../../functions/src/tui/render/llama_cpp_confirm_delete_shows_prompt.md)
- [llama_cpp_switching_shows_loading_message](../../../../functions/src/tui/render/llama_cpp_switching_shows_loading_message.md)
- [chat_input_renders_textarea_contents_and_hint](../../../../functions/src/tui/render/chat_input_renders_textarea_contents_and_hint.md)
- [model_info_panel_shows_provider_model_and_context_window](../../../../functions/src/tui/render/model_info_panel_shows_provider_model_and_context_window.md)
- [model_info_panel_shows_last_response_perf_metrics](../../../../functions/src/tui/render/model_info_panel_shows_last_response_perf_metrics.md)
- [help_screen_lists_commands_from_every_section](../../../../functions/src/tui/render/help_screen_lists_commands_from_every_section.md)
- [help_screen_shows_shift_enter_when_kitty_protocol_active](../../../../functions/src/tui/render/help_screen_shows_shift_enter_when_kitty_protocol_active.md)
- [chat_shows_pending_plan_banner_only_while_awaiting_approval](../../../../functions/src/tui/render/chat_shows_pending_plan_banner_only_while_awaiting_approval.md)
- [chat_message_thinking_block_toggles_between_collapsed_and_expanded](../../../../functions/src/tui/render/chat_message_thinking_block_toggles_between_collapsed_and_expanded.md)
- [chat_message_perf_footer_reports_cold_and_warm_starts](../../../../functions/src/tui/render/chat_message_perf_footer_reports_cold_and_warm_starts.md)
- [chat_shows_streaming_response_and_processing_indicator](../../../../functions/src/tui/render/chat_shows_streaming_response_and_processing_indicator.md)
- [chat_follows_the_end_of_a_reply_whose_paragraphs_wrap](../../../../functions/src/tui/render/chat_follows_the_end_of_a_reply_whose_paragraphs_wrap.md)
- [scrolling_up_hides_the_end_and_says_how_to_return](../../../../functions/src/tui/render/scrolling_up_hides_the_end_and_says_how_to_return.md)
- [an_empty_chat_shows_ferris_and_a_welcome](../../../../functions/src/tui/render/an_empty_chat_shows_ferris_and_a_welcome.md)
- [ferris_works_while_the_model_thinks](../../../../functions/src/tui/render/ferris_works_while_the_model_thinks.md)
- [a_new_session_costs_zero_not_minus_zero](../../../../functions/src/tui/render/a_new_session_costs_zero_not_minus_zero.md)
- [plan_mode_shows_full_document_with_tasks_and_criteria](../../../../functions/src/tui/render/plan_mode_shows_full_document_with_tasks_and_criteria.md)
- [plan_mode_shows_empty_state_without_a_plan](../../../../functions/src/tui/render/plan_mode_shows_empty_state_without_a_plan.md)
- [approval_dialog_shows_tool_name_capabilities_and_summarized_params](../../../../functions/src/tui/render/approval_dialog_shows_tool_name_capabilities_and_summarized_params.md)
- [approval_dialog_details_view_shows_pretty_printed_json](../../../../functions/src/tui/render/approval_dialog_details_view_shows_pretty_printed_json.md)
- [the_memories_screen_shows_its_filters_and_the_memory](../../../../functions/src/tui/render/the_memories_screen_shows_its_filters_and_the_memory.md)
- [the_memories_screen_scrolls_to_the_selection](../../../../functions/src/tui/render/the_memories_screen_scrolls_to_the_selection.md)