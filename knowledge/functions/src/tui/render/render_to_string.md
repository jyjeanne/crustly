---
type: Rust Function
title: render_to_string
resource: src/tui/render.rs#L2432-L2446
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/tui/render/render
  called_by:
  - functions/src/tui/render/header_shows_ollama_provider_badge_and_tokens_per_second
  - functions/src/tui/render/header_omits_tokens_per_second_when_unavailable
  - functions/src/tui/render/status_bar_shows_interactive_by_default
  - functions/src/tui/render/status_bar_shows_full_auto_when_active
  - functions/src/tui/render/skills_view_shows_name_and_description
  - functions/src/tui/render/skills_view_shows_empty_state_message
  - functions/src/tui/render/mcp_view_shows_connected_server_with_tool_count
  - functions/src/tui/render/mcp_view_shows_connection_error
  - functions/src/tui/render/mcp_view_shows_empty_state_message
  - functions/src/tui/render/model_download_dialog_shows_prompt_and_suggestions
  - functions/src/tui/render/model_download_progress_shows_status_and_bar
  - functions/src/tui/render/model_download_confirm_delete_shows_prompt
  - functions/src/tui/render/model_download_deleting_shows_status
  - functions/src/tui/render/llama_cpp_models_dialog_shows_prompt_and_local_models
  - functions/src/tui/render/llama_cpp_models_dialog_shows_loading_state
  - functions/src/tui/render/llama_cpp_download_progress_shows_status_and_bar
  - functions/src/tui/render/llama_cpp_confirm_delete_shows_prompt
  - functions/src/tui/render/llama_cpp_switching_shows_loading_message
  - functions/src/tui/render/chat_input_renders_textarea_contents_and_hint
  - functions/src/tui/render/model_info_panel_shows_provider_model_and_context_window
  - functions/src/tui/render/model_info_panel_shows_last_response_perf_metrics
  - functions/src/tui/render/help_screen_lists_commands_from_every_section
  - functions/src/tui/render/help_screen_shows_shift_enter_when_kitty_protocol_active
  - functions/src/tui/render/chat_shows_pending_plan_banner_only_while_awaiting_approval
  - functions/src/tui/render/chat_message_thinking_block_toggles_between_collapsed_and_expanded
  - functions/src/tui/render/chat_message_perf_footer_reports_cold_and_warm_starts
  - functions/src/tui/render/chat_shows_streaming_response_and_processing_indicator
  - functions/src/tui/render/plan_mode_shows_full_document_with_tasks_and_criteria
  - functions/src/tui/render/plan_mode_shows_empty_state_without_a_plan
  - functions/src/tui/render/approval_dialog_shows_tool_name_capabilities_and_summarized_params
  - functions/src/tui/render/approval_dialog_details_view_shows_pretty_printed_json
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
- [plan_mode_shows_full_document_with_tasks_and_criteria](../../../../functions/src/tui/render/plan_mode_shows_full_document_with_tasks_and_criteria.md)
- [plan_mode_shows_empty_state_without_a_plan](../../../../functions/src/tui/render/plan_mode_shows_empty_state_without_a_plan.md)
- [approval_dialog_shows_tool_name_capabilities_and_summarized_params](../../../../functions/src/tui/render/approval_dialog_shows_tool_name_capabilities_and_summarized_params.md)
- [approval_dialog_details_view_shows_pretty_printed_json](../../../../functions/src/tui/render/approval_dialog_details_view_shows_pretty_printed_json.md)