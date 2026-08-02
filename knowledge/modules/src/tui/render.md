---
type: Rust Module
title: render
resource: src/tui/render.rs#L1-L3038
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/super-app-app
  - external/super-events-appmode
  - external/super-markdown-parse-markdown-parse-plain-text
  - external/super-splash
  - external/crate-config-planexecmode
  - external/ratatui-layout-alignment-constraint-direction-layout-rect-style-color-modifier-style-text-line-span-widgets-block-borders-paragraph-wrap-frame
  - external/super
  - external/crate-db-database
  - external/crate-llm-agent-agentservice
  - external/crate-llm-provider-llmrequest-llmresponse-provider-providerstream-result-as-providerresult
  - external/crate-services-servicecontext
  - external/crate-tui-app-displaymessage
  - external/async-trait-async-trait
  - external/ratatui-backend-testbackend
  - external/ratatui-terminal
  - external/std-sync-arc
  - external/crate-llm-provider-perfmetrics
  member_of:
  - packages/crustly
---

# Contains

- [render](../../../functions/src/tui/render/render.md)
- [provider_icon](../../../functions/src/tui/render/provider_icon.md)
- [render_header](../../../functions/src/tui/render/render_header.md)
- [render_pending_plan_banner](../../../functions/src/tui/render/render_pending_plan_banner.md)
- [render_perf_footer](../../../functions/src/tui/render/render_perf_footer.md)
- [render_thinking_block](../../../functions/src/tui/render/render_thinking_block.md)
- [render_message_lines](../../../functions/src/tui/render/render_message_lines.md)
- [render_streaming_response](../../../functions/src/tui/render/render_streaming_response.md)
- [render_processing_indicator](../../../functions/src/tui/render/render_processing_indicator.md)
- [compute_scroll_offset](../../../functions/src/tui/render/compute_scroll_offset.md)
- [render_chat](../../../functions/src/tui/render/render_chat.md)
- [render_input](../../../functions/src/tui/render/render_input.md)
- [render_sessions](../../../functions/src/tui/render/render_sessions.md)
- [render_skills](../../../functions/src/tui/render/render_skills.md)
- [render_mcp](../../../functions/src/tui/render/render_mcp.md)
- [help_row](../../../functions/src/tui/render/help_row.md)
- [feature_row](../../../functions/src/tui/render/feature_row.md)
- [help_section_header](../../../functions/src/tui/render/help_section_header.md)
- [help_global_commands](../../../functions/src/tui/render/help_global_commands.md)
- [help_chat_mode](../../../functions/src/tui/render/help_chat_mode.md)
- [help_session_list](../../../functions/src/tui/render/help_session_list.md)
- [help_plan_mode](../../../functions/src/tui/render/help_plan_mode.md)
- [help_features](../../../functions/src/tui/render/help_features.md)
- [help_footer](../../../functions/src/tui/render/help_footer.md)
- [render_help](../../../functions/src/tui/render/render_help.md)
- [render_plan_help](../../../functions/src/tui/render/render_plan_help.md)
- [render_plan_task_lines](../../../functions/src/tui/render/render_plan_task_lines.md)
- [render_plan_document](../../../functions/src/tui/render/render_plan_document.md)
- [render_plan_empty_state](../../../functions/src/tui/render/render_plan_empty_state.md)
- [render_plan](../../../functions/src/tui/render/render_plan.md)
- [render_settings](../../../functions/src/tui/render/render_settings.md)
- [approval_dialog_area](../../../functions/src/tui/render/approval_dialog_area.md)
- [render_approval_header](../../../functions/src/tui/render/render_approval_header.md)
- [render_approval_capabilities](../../../functions/src/tui/render/render_approval_capabilities.md)
- [render_approval_input_detailed](../../../functions/src/tui/render/render_approval_input_detailed.md)
- [render_approval_input_summary](../../../functions/src/tui/render/render_approval_input_summary.md)
- [render_approval_actions](../../../functions/src/tui/render/render_approval_actions.md)
- [render_approval](../../../functions/src/tui/render/render_approval.md)
- [render_file_picker](../../../functions/src/tui/render/render_file_picker.md)
- [render_model_info](../../../functions/src/tui/render/render_model_info.md)
- [render_provider_switch](../../../functions/src/tui/render/render_provider_switch.md)
- [render_model_download](../../../functions/src/tui/render/render_model_download.md)
- [render_model_download_progress](../../../functions/src/tui/render/render_model_download_progress.md)
- [render_model_download_confirm_delete](../../../functions/src/tui/render/render_model_download_confirm_delete.md)
- [render_model_download_deleting](../../../functions/src/tui/render/render_model_download_deleting.md)
- [render_llama_cpp_models](../../../functions/src/tui/render/render_llama_cpp_models.md)
- [render_llama_cpp_download_progress](../../../functions/src/tui/render/render_llama_cpp_download_progress.md)
- [render_llama_cpp_confirm_delete](../../../functions/src/tui/render/render_llama_cpp_confirm_delete.md)
- [render_llama_cpp_deleting](../../../functions/src/tui/render/render_llama_cpp_deleting.md)
- [render_llama_cpp_switching](../../../functions/src/tui/render/render_llama_cpp_switching.md)
- [render_status_bar](../../../functions/src/tui/render/render_status_bar.md)
- [DummyProvider](../../../classes/src/tui/render/DummyProvider.md)
- [complete](../../../functions/src/tui/render/DummyProvider/provider/complete.md)
- [stream](../../../functions/src/tui/render/DummyProvider/provider/stream.md)
- [name](../../../functions/src/tui/render/DummyProvider/provider/name.md)
- [default_model](../../../functions/src/tui/render/DummyProvider/provider/default_model.md)
- [supported_models](../../../functions/src/tui/render/DummyProvider/provider/supported_models.md)
- [context_window](../../../functions/src/tui/render/DummyProvider/provider/context_window.md)
- [calculate_cost](../../../functions/src/tui/render/DummyProvider/provider/calculate_cost.md)
- [test_app](../../../functions/src/tui/render/test_app.md)
- [render_to_string](../../../functions/src/tui/render/render_to_string.md)
- [line_text](../../../functions/src/tui/render/line_text.md)
- [message_header_timestamp_is_shown_in_local_time](../../../functions/src/tui/render/message_header_timestamp_is_shown_in_local_time.md)
- [header_shows_ollama_provider_badge_and_tokens_per_second](../../../functions/src/tui/render/header_shows_ollama_provider_badge_and_tokens_per_second.md)
- [header_omits_tokens_per_second_when_unavailable](../../../functions/src/tui/render/header_omits_tokens_per_second_when_unavailable.md)
- [status_bar_shows_interactive_by_default](../../../functions/src/tui/render/status_bar_shows_interactive_by_default.md)
- [status_bar_shows_full_auto_when_active](../../../functions/src/tui/render/status_bar_shows_full_auto_when_active.md)
- [skills_view_shows_name_and_description](../../../functions/src/tui/render/skills_view_shows_name_and_description.md)
- [skills_view_shows_empty_state_message](../../../functions/src/tui/render/skills_view_shows_empty_state_message.md)
- [mcp_view_shows_connected_server_with_tool_count](../../../functions/src/tui/render/mcp_view_shows_connected_server_with_tool_count.md)
- [mcp_view_shows_connection_error](../../../functions/src/tui/render/mcp_view_shows_connection_error.md)
- [mcp_view_shows_empty_state_message](../../../functions/src/tui/render/mcp_view_shows_empty_state_message.md)
- [model_download_dialog_shows_prompt_and_suggestions](../../../functions/src/tui/render/model_download_dialog_shows_prompt_and_suggestions.md)
- [model_download_progress_shows_status_and_bar](../../../functions/src/tui/render/model_download_progress_shows_status_and_bar.md)
- [model_download_confirm_delete_shows_prompt](../../../functions/src/tui/render/model_download_confirm_delete_shows_prompt.md)
- [model_download_deleting_shows_status](../../../functions/src/tui/render/model_download_deleting_shows_status.md)
- [llama_cpp_models_dialog_shows_prompt_and_local_models](../../../functions/src/tui/render/llama_cpp_models_dialog_shows_prompt_and_local_models.md)
- [llama_cpp_models_dialog_shows_loading_state](../../../functions/src/tui/render/llama_cpp_models_dialog_shows_loading_state.md)
- [llama_cpp_download_progress_shows_status_and_bar](../../../functions/src/tui/render/llama_cpp_download_progress_shows_status_and_bar.md)
- [llama_cpp_confirm_delete_shows_prompt](../../../functions/src/tui/render/llama_cpp_confirm_delete_shows_prompt.md)
- [llama_cpp_switching_shows_loading_message](../../../functions/src/tui/render/llama_cpp_switching_shows_loading_message.md)
- [chat_input_renders_textarea_contents_and_hint](../../../functions/src/tui/render/chat_input_renders_textarea_contents_and_hint.md)
- [model_info_panel_shows_provider_model_and_context_window](../../../functions/src/tui/render/model_info_panel_shows_provider_model_and_context_window.md)
- [model_info_panel_shows_last_response_perf_metrics](../../../functions/src/tui/render/model_info_panel_shows_last_response_perf_metrics.md)
- [help_screen_lists_commands_from_every_section](../../../functions/src/tui/render/help_screen_lists_commands_from_every_section.md)
- [help_screen_shows_shift_enter_when_kitty_protocol_active](../../../functions/src/tui/render/help_screen_shows_shift_enter_when_kitty_protocol_active.md)
- [chat_shows_pending_plan_banner_only_while_awaiting_approval](../../../functions/src/tui/render/chat_shows_pending_plan_banner_only_while_awaiting_approval.md)
- [chat_message_thinking_block_toggles_between_collapsed_and_expanded](../../../functions/src/tui/render/chat_message_thinking_block_toggles_between_collapsed_and_expanded.md)
- [chat_message_perf_footer_reports_cold_and_warm_starts](../../../functions/src/tui/render/chat_message_perf_footer_reports_cold_and_warm_starts.md)
- [chat_shows_streaming_response_and_processing_indicator](../../../functions/src/tui/render/chat_shows_streaming_response_and_processing_indicator.md)
- [plan_mode_shows_full_document_with_tasks_and_criteria](../../../functions/src/tui/render/plan_mode_shows_full_document_with_tasks_and_criteria.md)
- [plan_mode_shows_empty_state_without_a_plan](../../../functions/src/tui/render/plan_mode_shows_empty_state_without_a_plan.md)
- [test_approval_request](../../../functions/src/tui/render/test_approval_request.md)
- [approval_dialog_shows_tool_name_capabilities_and_summarized_params](../../../functions/src/tui/render/approval_dialog_shows_tool_name_capabilities_and_summarized_params.md)
- [approval_dialog_details_view_shows_pretty_printed_json](../../../functions/src/tui/render/approval_dialog_details_view_shows_pretty_printed_json.md)

# Imports

- `super::app::App`
- `super::events::AppMode`
- `super::markdown::{parse_markdown, parse_plain_text}`
- `super::splash`
- `crate::config::PlanExecMode`
- `ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
}`
- `super::*`
- `crate::db::Database`
- `crate::llm::agent::AgentService`
- `crate::llm::provider::{
        LLMRequest, LLMResponse, Provider, ProviderStream, Result as ProviderResult,
    }`
- `crate::services::ServiceContext`
- `crate::tui::app::DisplayMessage`
- `async_trait::async_trait`
- `ratatui::backend::TestBackend`
- `ratatui::Terminal`
- `std::sync::Arc`
- `crate::llm::provider::PerfMetrics`

# Member of

- [crustly](../../../packages/crustly.md)